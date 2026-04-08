package com.humanizar.units.exception;

import com.humanizar.units.model.enums.ReasonCode;

public class UnitException extends RuntimeException {

    private final ReasonCode reasonCode;
    private final String message;

    public UnitException(ReasonCode reasonCode, String message) {
        super(resolveMessage(reasonCode, message));
        this.reasonCode = reasonCode;
        this.message = resolveMessage(reasonCode, message);
    }

    public UnitException(ReasonCode reasonCode) {
        this(reasonCode, null);
    }

    public int getStatusCode() {
        return reasonCode != null ? reasonCode.getStatusCode() : 500;
    }

    public ReasonCode getReasonCode() {
        return reasonCode;
    }

    @Override
    public String getMessage() {
        return message;
    }

    public boolean isRetryable() {
        return reasonCode != null && reasonCode.isRetryable();
    }

    private static String resolveMessage(ReasonCode reasonCode, String message) {
        if (message != null && !message.isBlank()) {
            return message;
        }
        if (reasonCode != null) {
            return reasonCode.getMessage();
        }
        return null;
    }
}
