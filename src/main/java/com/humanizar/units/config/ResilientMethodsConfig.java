package com.humanizar.units.config;

import java.lang.annotation.Documented;
import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;
import java.lang.reflect.Method;

import org.springframework.aot.hint.MemberCategory;
import org.springframework.aot.hint.RuntimeHints;
import org.springframework.aot.hint.RuntimeHintsRegistrar;
import org.springframework.context.annotation.Configuration;
import org.springframework.context.annotation.ImportRuntimeHints;
import org.springframework.dao.RecoverableDataAccessException;
import org.springframework.dao.TransientDataAccessException;
import org.springframework.resilience.annotation.Retryable;
import org.springframework.resilience.retry.MethodRetryPredicate;
import org.springframework.transaction.CannotCreateTransactionException;

import com.humanizar.units.exception.Throwables;

import jakarta.persistence.QueryTimeoutException;

@Configuration
@ImportRuntimeHints(ResilientMethodsConfig.ResilientMethodsRuntimeHints.class)
public class ResilientMethodsConfig {

    public static final long RETRIEVE_MAX_RETRIES = 2L;
    public static final String RETRIEVE_TIMEOUT = "30s";

    @Target(ElementType.METHOD)
    @Retention(RetentionPolicy.RUNTIME)
    @Documented
    @Retryable(maxRetries = RETRIEVE_MAX_RETRIES, timeoutString = RETRIEVE_TIMEOUT, predicate = RetrieveTransientRetryPredicate.class)
    public @interface Retry {
    }

    public static class RetrieveTransientRetryPredicate implements MethodRetryPredicate {

        @Override
        public boolean shouldRetry(Method method, Throwable throwable) {
            return Throwables.hasCause(
                    throwable,
                    TransientDataAccessException.class,
                    RecoverableDataAccessException.class,
                    CannotCreateTransactionException.class,
                    QueryTimeoutException.class);
        }
    }

    public static class ResilientMethodsRuntimeHints implements RuntimeHintsRegistrar {

        @Override
        public void registerHints(RuntimeHints hints, ClassLoader classLoader) {
            hints.reflection().registerType(
                    RetrieveTransientRetryPredicate.class,
                    MemberCategory.INVOKE_DECLARED_CONSTRUCTORS);
        }
    }
}
