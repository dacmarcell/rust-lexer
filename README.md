# Analisador Lexico criado na Linguagem Rust feito para a linguagem PL

| Implementado? | Exemplo de Literal | Tipo de Token |
|---------------|--------------------|---------------|
| SIM           | 'single quote'     | String        |
| SIM           | "double quote"     | String        |
| SIM           | 123                | Number        |
| SIM           | 123.456            | Number        |
| SIM           | =                  | Assign        |
| NAO           | +                  | Assign        |
| SIM           | let                | Identifier    |
| NAO           | const              | Identifier    |
