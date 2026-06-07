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
import jakarta.persistence.UniqueConstraint;

@Entity
@Table(name = "municipio", uniqueConstraints = {
        @UniqueConstraint(name = "uk_municipio_codigo_ibge", columnNames = "codigo_ibge")
})
public class Municipio {

    @Id
    @GeneratedValue(strategy = GenerationType.UUID)
    private UUID id;

    @Column(name = "codigo_ibge", nullable = false, length = 7)
    private String codigoIbge;

    @Column(name = "nome", nullable = false)
    private String nome;

    @Column(name = "uf", nullable = false, length = 2)
    private String uf;

    @CreationTimestamp
    @Column(name = "created_at", nullable = false)
    private LocalDateTime createdAt;

    @UpdateTimestamp
    @Column(name = "updated_at", nullable = false)
    private LocalDateTime updatedAt;

    public Municipio() {
    }

    public Municipio(UUID id, String codigoIbge, String nome, String uf, LocalDateTime createdAt,
            LocalDateTime updatedAt) {
        this.id = id;
        this.codigoIbge = codigoIbge;
        this.nome = nome;
        this.uf = uf;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }

    public Municipio(String codigoIbge, String nome, String uf) {
        this(null, codigoIbge, nome, uf, null, null);
    }

    public UUID getId() {
        return id;
    }

    public void setId(UUID id) {
        this.id = id;
    }

    public String getCodigoIbge() {
        return codigoIbge;
    }

    public void setCodigoIbge(String codigoIbge) {
        this.codigoIbge = codigoIbge;
    }

    public String getNome() {
        return nome;
    }

    public void setNome(String nome) {
        this.nome = nome;
    }

    public String getUf() {
        return uf;
    }

    public void setUf(String uf) {
        this.uf = uf;
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
    public boolean equals(Object other) {
        if (this == other) {
            return true;
        }
        if (other == null || getClass() != other.getClass()) {
            return false;
        }
        Municipio municipio = (Municipio) other;
        return Objects.equals(id, municipio.id);
    }

    @Override
    public int hashCode() {
        return Objects.hash(id);
    }

    public static Builder builder() {
        return new Builder();
    }

    public static class Builder {
        private UUID id;
        private String codigoIbge;
        private String nome;
        private String uf;
        private LocalDateTime createdAt;
        private LocalDateTime updatedAt;

        public Builder id(UUID id) {
            this.id = id;
            return this;
        }

        public Builder codigoIbge(String codigoIbge) {
            this.codigoIbge = codigoIbge;
            return this;
        }

        public Builder nome(String nome) {
            this.nome = nome;
            return this;
        }

        public Builder uf(String uf) {
            this.uf = uf;
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

        public Municipio build() {
            return new Municipio(id, codigoIbge, nome, uf, createdAt, updatedAt);
        }
    }

    @Override
    public String toString() {
        return "Municipio [id=" + id +
                ", codigoIbge=" + codigoIbge +
                ", nome=" + nome +
                ", uf=" + uf +
                ", createdAt=" + createdAt +
                ", updatedAt=" + updatedAt + "]";
    }
}
