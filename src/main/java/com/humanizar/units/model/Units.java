package com.humanizar.units.model;

import java.time.LocalDateTime;
import java.util.Objects;
import java.util.UUID;

import org.hibernate.annotations.CreationTimestamp;
import org.hibernate.annotations.UpdateTimestamp;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.Table;

@Entity
@Table(name = "units")
public class Units {

    @Id
    @GeneratedValue(strategy = GenerationType.UUID)
    private UUID id;

    @Column(name = "unit_name", nullable = false)
    private String unitName;

    @Column(name = "razao_social", nullable = false)
    private String razaoSocial;

    @Column(name = "endereco", nullable = false)
    private String endereco;

    @Column(name = "numero", nullable = false)
    private String numero;

    @Column(name = "complemento")
    private String complemento;

    @Column(name = "bairro", nullable = false)
    private String bairro;

    @Column(name = "cidade", nullable = false)
    private String cidade;

    @Column(name = "estado", nullable = false)
    private String estado;

    @Column(name = "cep", nullable = false)
    private String cep;

    @Column(name = "cnpj", nullable = false)
    private String cnpj;

    @CreationTimestamp
    @Column(name = "created_at", nullable = false)
    private LocalDateTime createdAt;

    @UpdateTimestamp
    @Column(name = "updated_at", nullable = false)
    private LocalDateTime updatedAt;

    public Units() {
    }

    public Units(UUID id, String unitName, String razaoSocial, String endereco, String numero, String complemento,
            String bairro, String cidade, String estado, String cep, String cnpj, LocalDateTime createdAt,
            LocalDateTime updatedAt) {
        this.id = id;
        this.unitName = unitName;
        this.razaoSocial = razaoSocial;
        this.endereco = endereco;
        this.numero = numero;
        this.complemento = complemento;
        this.bairro = bairro;
        this.cidade = cidade;
        this.estado = estado;
        this.cep = cep;
        this.cnpj = cnpj;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }

    public Units(String unitName, String razaoSocial, String endereco, String numero, String complemento,
            String bairro, String cidade, String estado, String cep, String cnpj) {
        this(null, unitName, razaoSocial, endereco, numero, complemento, bairro, cidade, estado, cep, cnpj, null,
                null);
    }

    public UUID getId() {
        return id;
    }

    public void setId(UUID id) {
        this.id = id;
    }

    public String getUnitName() {
        return unitName;
    }

    public void setUnitName(String unitName) {
        this.unitName = unitName;
    }

    public String getRazaoSocial() {
        return razaoSocial;
    }

    public void setRazaoSocial(String razaoSocial) {
        this.razaoSocial = razaoSocial;
    }

    public String getEndereco() {
        return endereco;
    }

    public void setEndereco(String endereco) {
        this.endereco = endereco;
    }

    public String getNumero() {
        return numero;
    }

    public void setNumero(String numero) {
        this.numero = numero;
    }

    public String getComplemento() {
        return complemento;
    }

    public void setComplemento(String complemento) {
        this.complemento = complemento;
    }

    public String getBairro() {
        return bairro;
    }

    public void setBairro(String bairro) {
        this.bairro = bairro;
    }

    public String getCidade() {
        return cidade;
    }

    public void setCidade(String cidade) {
        this.cidade = cidade;
    }

    public String getEstado() {
        return estado;
    }

    public void setEstado(String estado) {
        this.estado = estado;
    }

    public String getCep() {
        return cep;
    }

    public void setCep(String cep) {
        this.cep = cep;
    }

    public String getCnpj() {
        return cnpj;
    }

    public void setCnpj(String cnpj) {
        this.cnpj = cnpj;
    }

    public LocalDateTime getCreatedAt() {
        return createdAt;
    }

    public void setCreatedAt(LocalDateTime createdAt) {
        this.createdAt = createdAt;
    }

    public LocalDateTime getUpdatedAt() {
        return updatedAt;
    }

    public void setUpdatedAt(LocalDateTime updatedAt) {
        this.updatedAt = updatedAt;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) {
            return true;
        }
        if (o == null || getClass() != o.getClass()) {
            return false;
        }
        Units units = (Units) o;
        return Objects.equals(id, units.id) &&
                Objects.equals(unitName, units.unitName) &&
                Objects.equals(razaoSocial, units.razaoSocial) &&
                Objects.equals(endereco, units.endereco) &&
                Objects.equals(numero, units.numero) &&
                Objects.equals(complemento, units.complemento) &&
                Objects.equals(bairro, units.bairro) &&
                Objects.equals(cidade, units.cidade) &&
                Objects.equals(estado, units.estado) &&
                Objects.equals(cep, units.cep) &&
                Objects.equals(cnpj, units.cnpj) &&
                Objects.equals(createdAt, units.createdAt) &&
                Objects.equals(updatedAt, units.updatedAt);
    }

    @Override
    public int hashCode() {
        return Objects.hash(id, unitName, razaoSocial, endereco, numero, complemento, bairro, cidade, estado, cep,
                cnpj, createdAt, updatedAt);
    }

    public static Builder builder() {
        return new Builder();
    }

    public static class Builder {
        private UUID id;
        private String unitName;
        private String razaoSocial;
        private String endereco;
        private String numero;
        private String complemento;
        private String bairro;
        private String cidade;
        private String estado;
        private String cep;
        private String cnpj;
        private LocalDateTime createdAt;
        private LocalDateTime updatedAt;

        public Builder id(UUID id) {
            this.id = id;
            return this;
        }

        public Builder unitName(String unitName) {
            this.unitName = unitName;
            return this;
        }

        public Builder razaoSocial(String razaoSocial) {
            this.razaoSocial = razaoSocial;
            return this;
        }

        public Builder endereco(String endereco) {
            this.endereco = endereco;
            return this;
        }

        public Builder numero(String numero) {
            this.numero = numero;
            return this;
        }

        public Builder complemento(String complemento) {
            this.complemento = complemento;
            return this;
        }

        public Builder bairro(String bairro) {
            this.bairro = bairro;
            return this;
        }

        public Builder cidade(String cidade) {
            this.cidade = cidade;
            return this;
        }

        public Builder estado(String estado) {
            this.estado = estado;
            return this;
        }

        public Builder cep(String cep) {
            this.cep = cep;
            return this;
        }

        public Builder cnpj(String cnpj) {
            this.cnpj = cnpj;
            return this;
        }

        public Builder createdAt(LocalDateTime createdAt) {
            this.createdAt = createdAt;
            return this;
        }

        public Builder updatedAt(LocalDateTime updatedAt) {
            this.updatedAt = updatedAt;
            return this;
        }

        public Units build() {
            return new Units(id, unitName, razaoSocial, endereco, numero, complemento, bairro, cidade, estado, cep,
                    cnpj, createdAt, updatedAt);
        }
    }

    @Override
    public String toString() {
        return "Units [id=" + id +
                ", unitName=" + unitName +
                ", razaoSocial=" + razaoSocial +
                ", endereco=" + endereco +
                ", numero=" + numero +
                ", complemento=" + complemento +
                ", bairro=" + bairro +
                ", cidade=" + cidade +
                ", estado=" + estado +
                ", cep=" + cep +
                ", cnpj=" + cnpj +
                ", createdAt=" + createdAt +
                ", updatedAt=" + updatedAt + "]";
    }
}
