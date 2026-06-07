package com.humanizar.units.exception;

/**
 * Travessia da cadeia para exceptions.
 * Centraliza a lógica entre o handler global e o retry.
 */
public final class Throwables {

    private Throwables() {
    }

    public static String rootCauseMessage(Throwable throwable) {
        Throwable current = throwable;
        while (current != null && current.getCause() != null && current.getCause() != current) {
            current = current.getCause();
        }
        if (current == null || current.getMessage() == null) {
            return "";
        }
        return current.getMessage();
    }

    @SafeVarargs
    public static boolean hasCause(Throwable throwable, Class<? extends Throwable>... causeTypes) {
        Throwable current = throwable;
        while (current != null) {
            for (Class<? extends Throwable> causeType : causeTypes) {
                if (causeType.isInstance(current)) {
                    return true;
                }
            }
            Throwable cause = current.getCause();
            current = cause != current ? cause : null;
        }
        return false;
    }
}
