package com.humanizar.units;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.resilience.annotation.EnableResilientMethods;

@SpringBootApplication
@EnableResilientMethods
public class HumanizarUnitsApplication {

    static void main(String[] args) {
        SpringApplication.run(HumanizarUnitsApplication.class, args);
    }

}
