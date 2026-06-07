package com.humanizar.units.controller.handler;

import java.time.OffsetDateTime;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.access.AccessDeniedException;
import org.springframework.security.core.AuthenticationException;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.RestControllerAdvice;

import com.humanizar.units.controller.dto.UnitErrorResponseDTO;
import com.humanizar.units.exception.Throwables;
import com.humanizar.units.exception.UnitException;
import com.humanizar.units.model.enums.ReasonCode;

import jakarta.servlet.http.HttpServletRequest;

@RestControllerAdvice
public class UnitExceptionHandler {

    private static final Logger log = LoggerFactory.getLogger(UnitExceptionHandler.class);

    @ExceptionHandler(UnitException.class)
    public ResponseEntity<UnitErrorResponseDTO> handleUnitException(
            UnitException exception,
            HttpServletRequest request) {
        return buildResponse(
                exception.getStatusCode(),
                exception.getReasonCode() != null ? exception.getReasonCode().name() : ReasonCode.PERSISTENCE_FAILURE.name(),
                exception.getMessage(),
                request,
                exception);
    }

    @ExceptionHandler(AuthenticationException.class)
    public ResponseEntity<UnitErrorResponseDTO> handleAuthenticationException(
            AuthenticationException exception,
            HttpServletRequest request) {
        return buildResponse(
                ReasonCode.AUTHENTICATION_FAILURE.getStatusCode(),
                ReasonCode.AUTHENTICATION_FAILURE.name(),
                ReasonCode.AUTHENTICATION_FAILURE.getMessage(),
                request,
                exception);
    }

    @ExceptionHandler(AccessDeniedException.class)
    public ResponseEntity<UnitErrorResponseDTO> handleAccessDeniedException(
            AccessDeniedException exception,
            HttpServletRequest request) {
        return buildResponse(
                ReasonCode.AUTHORIZATION_FAILURE.getStatusCode(),
                ReasonCode.AUTHORIZATION_FAILURE.name(),
                ReasonCode.AUTHORIZATION_FAILURE.getMessage(),
                request,
                exception);
    }

    @ExceptionHandler(RuntimeException.class)
    public ResponseEntity<UnitErrorResponseDTO> handleRuntimeException(
            RuntimeException exception,
            HttpServletRequest request) {
        return buildResponse(
                ReasonCode.PERSISTENCE_FAILURE.getStatusCode(),
                ReasonCode.PERSISTENCE_FAILURE.name(),
                ReasonCode.PERSISTENCE_FAILURE.getMessage(),
                request,
                exception);
    }

    private ResponseEntity<UnitErrorResponseDTO> buildResponse(
            int status,
            String reasonCode,
            String message,
            HttpServletRequest request,
            Throwable exception) {
        String path = request != null ? request.getRequestURI() : null;
        logException(status, reasonCode, path, exception);

        UnitErrorResponseDTO body = new UnitErrorResponseDTO(
                status,
                reasonCode,
                message,
                path,
                OffsetDateTime.now());

        return ResponseEntity.status(resolveStatus(status)).body(body);
    }

    private HttpStatus resolveStatus(int statusCode) {
        HttpStatus status = HttpStatus.resolve(statusCode);
        return status != null ? status : HttpStatus.INTERNAL_SERVER_ERROR;
    }

    private void logException(
            int status,
            String reasonCode,
            String path,
            Throwable exception) {
        String logMessage = "Erro no processamento HTTP. reasonCode={}, status={}, path={}, causa={}";
        String rootCauseMessage = Throwables.rootCauseMessage(exception);

        if (status >= 500) {
            log.error(logMessage, reasonCode, status, path, rootCauseMessage, exception);
            return;
        }

        log.warn(logMessage, reasonCode, status, path, rootCauseMessage);
    }
}
