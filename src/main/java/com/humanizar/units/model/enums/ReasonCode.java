package com.humanizar.units.model.enums;

public enum ReasonCode {

    UNIT_NOT_FOUND(404, "Unidade não encontrada.", false),
    MUNICIPIO_NOT_FOUND(404, "Município não encontrado.", false),
    MUNICIPIO_DUPLICATED(409, "Município já cadastrado.", false),
    MUNICIPIO_HAS_UNITS(409, "Município possui unidades vinculadas.", false),
    VALIDATION_ERROR(400, "Falha de validação da solicitação.", false),
    AUTHENTICATION_FAILURE(401, "Falha de autenticação: credenciais ausentes ou inválidas.", false),
    AUTHORIZATION_FAILURE(403, "Falha de autorização: acesso negado para este recurso.", false),
    PERSISTENCE_FAILURE(503, "Falha de persistência no banco de dados.", true);

    private final int statusCode;
    private final String message;
    private final boolean retryable;

    ReasonCode(int statusCode, String message, boolean retryable) {
        this.statusCode = statusCode;
        this.message = message;
        this.retryable = retryable;
    }

    public int getStatusCode() {
        return statusCode;
    }

    public String getMessage() {
        return message;
    }

    public boolean isRetryable() {
        return retryable;
    }
}
