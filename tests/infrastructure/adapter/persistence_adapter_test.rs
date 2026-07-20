#![forbid(unsafe_code)]

use std::error::Error;
use std::io::Error as IoError;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use humanizar_units::domain::model::{Municipio, Unit};
use humanizar_units::domain::port::{MunicipioPort, UnitPort};
use humanizar_units::infrastructure::adapter::{MunicipioAdapter, UnitAdapter};
use humanizar_units::infrastructure::config::{DatabaseConfig, RetryConfig};
use humanizar_units::infrastructure::persistence::{MunicipioRepository, UnitRepository};

type TestResult<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::test]
async fn adapters_execute_the_complete_crud_contract_on_postgresql() -> TestResult {
    let database = DatabaseConfig::from_env().await?;
    let retry_executor = RetryConfig::from_env()?.executor();
    let municipio_port: Arc<dyn MunicipioPort> = Arc::new(MunicipioAdapter::new(
        MunicipioRepository::new(database.clone()),
        retry_executor.clone(),
    ));
    let unit_port: Arc<dyn UnitPort> = Arc::new(UnitAdapter::new(
        UnitRepository::new(database.clone()),
        retry_executor,
    ));
    let suffix = unique_suffix();
    let codigo_ibge = format!("{suffix:07}");
    let cnpj = format!("99{suffix:07}000190");

    cleanup(&database, &codigo_ibge, &cnpj).await?;
    let scenario = execute_scenario(
        Arc::clone(&municipio_port),
        Arc::clone(&unit_port),
        &codigo_ibge,
        &cnpj,
    )
    .await;
    let cleanup_result = cleanup(&database, &codigo_ibge, &cnpj).await;

    scenario?;
    cleanup_result?;
    Ok(())
}

async fn execute_scenario(
    municipio_port: Arc<dyn MunicipioPort>,
    unit_port: Arc<dyn UnitPort>,
    codigo_ibge: &str,
    cnpj: &str,
) -> TestResult {
    let municipio = municipio_port
        .save(Municipio::new(codigo_ibge, "Municipio Teste", "SP"))
        .await?;
    let municipio_id = required_id(municipio.id(), "municipio salvo deve possuir ID")?;
    ensure(
        municipio.created_at().is_some(),
        "municipio salvo deve possuir created_at",
    )?;
    ensure(
        municipio.updated_at().is_some(),
        "municipio salvo deve possuir updated_at",
    )?;

    let found_by_id = municipio_port.find_by_id(municipio_id).await?;
    ensure(
        found_by_id == Some(municipio.clone()),
        "busca de municipio por ID divergiu",
    )?;
    let found_by_code = municipio_port.find_by_codigo_ibge(codigo_ibge).await?;
    ensure(
        found_by_code == Some(municipio.clone()),
        "busca de municipio por codigo IBGE divergiu",
    )?;
    ensure(
        municipio_port
            .find_all()
            .await?
            .iter()
            .any(|item| item.id() == Some(municipio_id)),
        "listagem nao retornou o municipio criado",
    )?;

    let updated_municipio = municipio_port
        .save(Municipio::restore(
            municipio.id(),
            municipio.codigo_ibge(),
            "Municipio Teste Atualizado",
            municipio.uf(),
            municipio.created_at(),
            municipio.updated_at(),
        ))
        .await?;
    ensure(
        updated_municipio.nome() == "Municipio Teste Atualizado",
        "atualizacao do municipio nao foi persistida",
    )?;

    let unit = unit_port
        .save(Unit::new(
            municipio_id,
            "Unidade Teste",
            "Humanizar Teste Ltda",
            "Rua de Teste",
            "100",
            None,
            "Centro",
            "01001000",
            cnpj,
        ))
        .await?;
    let unit_id = required_id(unit.id(), "unidade salva deve possuir ID")?;
    ensure(
        unit.created_at().is_some(),
        "unidade salva deve possuir created_at",
    )?;
    ensure(
        unit_port.count_by_municipio_id(municipio_id).await? == 1,
        "contagem de unidades deve ser um",
    )?;
    ensure(
        unit_port
            .find_by_id_and_municipio_id(unit_id, municipio_id)
            .await?
            == Some(unit.clone()),
        "busca da unidade por ID e municipio divergiu",
    )?;
    ensure(
        unit_port.find_by_municipio_id(municipio_id).await?.len() == 1,
        "listagem por municipio deve retornar uma unidade",
    )?;
    ensure(
        unit_port
            .find_by_municipio_id_and_cnpj(municipio_id, cnpj)
            .await?
            == Some(unit.clone()),
        "busca por municipio e CNPJ divergiu",
    )?;
    ensure(
        unit_port.find_by_ids(&[unit_id]).await?.len() == 1,
        "listagem por IDs deve retornar uma unidade",
    )?;
    ensure(
        unit_port.find_by_ids(&[]).await?.is_empty(),
        "listagem por IDs vazios deve retornar vazio",
    )?;

    let updated_unit = unit_port
        .save(Unit::restore(
            unit.id(),
            unit.municipio_id(),
            "Unidade Teste Atualizada",
            unit.razao_social(),
            unit.endereco(),
            unit.numero(),
            Some("Sala 2".to_owned()),
            unit.bairro(),
            unit.cep(),
            unit.cnpj(),
            unit.created_at(),
            unit.updated_at(),
        ))
        .await?;
    ensure(
        updated_unit.unit_name() == "Unidade Teste Atualizada",
        "atualizacao da unidade nao foi persistida",
    )?;
    ensure(
        updated_unit.complemento() == Some("Sala 2"),
        "complemento atualizado divergiu",
    )?;

    ensure(
        unit_port.delete_by_id(unit_id).await?,
        "primeira exclusao da unidade deve remover uma linha",
    )?;
    ensure(
        !unit_port.delete_by_id(unit_id).await?,
        "segunda exclusao da unidade nao deve remover linha",
    )?;
    ensure(
        unit_port.count_by_municipio_id(municipio_id).await? == 0,
        "contagem deve ser zero apos exclusao da unidade",
    )?;
    ensure(
        municipio_port.delete_by_id(municipio_id).await?,
        "primeira exclusao do municipio deve remover uma linha",
    )?;
    ensure(
        !municipio_port.delete_by_id(municipio_id).await?,
        "segunda exclusao do municipio nao deve remover linha",
    )?;
    Ok(())
}

async fn cleanup(database: &DatabaseConfig, codigo_ibge: &str, cnpj: &str) -> TestResult {
    let connection = database.acquire().await?;
    connection
        .execute("DELETE FROM public.units WHERE cnpj = $1", &[&cnpj])
        .await?;
    connection
        .execute(
            "DELETE FROM public.municipio WHERE codigo_ibge = $1",
            &[&codigo_ibge],
        )
        .await?;
    Ok(())
}

fn unique_suffix() -> u32 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("relogio deve estar apos o epoch")
        .subsec_nanos();

    nanos % 10_000_000
}

fn required_id(id: Option<uuid::Uuid>, message: &'static str) -> TestResult<uuid::Uuid> {
    id.ok_or_else(|| IoError::other(message).into())
}

fn ensure(condition: bool, message: &'static str) -> TestResult {
    if condition {
        return Ok(());
    }

    Err(IoError::other(message).into())
}
