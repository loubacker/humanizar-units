package com.humanizar.units.config;

import java.io.IOException;
import java.time.OffsetDateTime;
import java.util.Collection;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Locale;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.http.MediaType;
import org.springframework.security.config.annotation.method.configuration.EnableMethodSecurity;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.annotation.web.configuration.EnableWebSecurity;
import org.springframework.security.config.annotation.web.configurers.AbstractHttpConfigurer;
import org.springframework.security.config.http.SessionCreationPolicy;
import org.springframework.security.core.GrantedAuthority;
import org.springframework.security.core.authority.SimpleGrantedAuthority;
import org.springframework.security.oauth2.jwt.Jwt;
import org.springframework.security.oauth2.server.resource.authentication.JwtAuthenticationConverter;
import org.springframework.security.web.SecurityFilterChain;
import org.springframework.util.StringUtils;
import org.springframework.web.cors.CorsConfigurationSource;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.humanizar.units.controller.dto.UnitErrorResponseDTO;
import com.humanizar.units.model.enums.ReasonCode;

import jakarta.servlet.http.HttpServletResponse;

@Configuration
@EnableWebSecurity
@EnableMethodSecurity
public class SecurityConfig {

    private final CorsConfigurationSource corsConfigurationSource;
    private final ObjectMapper objectMapper;

    public SecurityConfig(CorsConfigurationSource corsConfigurationSource, ObjectMapper objectMapper) {
        this.corsConfigurationSource = corsConfigurationSource;
        this.objectMapper = objectMapper;
    }

    @Bean
    public SecurityFilterChain securityFilterChain(HttpSecurity http) throws Exception {
        http
                .cors(cors -> cors.configurationSource(corsConfigurationSource))
                .csrf(AbstractHttpConfigurer::disable)
                .sessionManagement(session -> session.sessionCreationPolicy(SessionCreationPolicy.STATELESS))
                .exceptionHandling(exceptions -> exceptions
                        .authenticationEntryPoint((request, response, exception) -> writeSecurityError(
                                response,
                                request.getRequestURI(),
                                ReasonCode.AUTHENTICATION_FAILURE))
                        .accessDeniedHandler((request, response, exception) -> writeSecurityError(
                                response,
                                request.getRequestURI(),
                                ReasonCode.AUTHORIZATION_FAILURE)))
                .authorizeHttpRequests(auth -> auth
                        .requestMatchers("/api/v1/**").authenticated()
                        .requestMatchers("/actuator/**").permitAll()
                        .anyRequest().authenticated())
                .oauth2ResourceServer(oauth2 -> oauth2
                        .jwt(jwt -> jwt.jwtAuthenticationConverter(jwtAuthenticationConverter())));

        return http.build();
    }

    @Bean
    public JwtAuthenticationConverter jwtAuthenticationConverter() {
        JwtAuthenticationConverter converter = new JwtAuthenticationConverter();
        converter.setJwtGrantedAuthoritiesConverter(this::extractGrantedAuthorities);
        return converter;
    }

    private Collection<GrantedAuthority> extractGrantedAuthorities(Jwt jwt) {
        Set<String> roles = new LinkedHashSet<>();
        roles.addAll(extractRoleValues(jwt.getClaims().get("role")));
        roles.addAll(extractRoleValues(jwt.getClaims().get("roles")));

        return roles.stream()
                .filter(StringUtils::hasText)
                .map(this::normalizeRoleAuthority)
                .map(SimpleGrantedAuthority::new)
                .collect(Collectors.toList());
    }

    private List<String> extractRoleValues(Object claimValue) {
        if (claimValue == null) {
            return List.of();
        }

        if (claimValue instanceof String claimAsString) {
            if (claimAsString.contains(",")) {
                return List.of(claimAsString.split(","));
            }
            return List.of(claimAsString);
        }

        if (claimValue instanceof Collection<?> claimAsCollection) {
            return claimAsCollection.stream()
                    .map(String::valueOf)
                    .toList();
        }

        if (claimValue instanceof Object[] claimAsArray) {
            return Stream.of(claimAsArray)
                    .map(String::valueOf)
                    .toList();
        }

        return List.of(String.valueOf(claimValue));
    }

    private String normalizeRoleAuthority(String rawRole) {
        String normalizedRole = rawRole.trim();
        if (normalizedRole.startsWith("ROLE_")) {
            normalizedRole = normalizedRole.substring("ROLE_".length());
        }
        return "ROLE_" + normalizedRole.toUpperCase(Locale.ROOT);
    }

    private void writeSecurityError(
            HttpServletResponse response,
            String path,
            ReasonCode reasonCode) throws IOException {
        if (response.isCommitted()) {
            return;
        }

        UnitErrorResponseDTO payload = new UnitErrorResponseDTO(
                reasonCode.getStatusCode(),
                reasonCode.name(),
                reasonCode.getMessage(),
                path,
                OffsetDateTime.now());

        response.setStatus(reasonCode.getStatusCode());
        response.setContentType(MediaType.APPLICATION_JSON_VALUE);
        objectMapper.writeValue(response.getOutputStream(), payload);
    }
}
