# A lexer for Java lang

| Implemented? | Literal Example | Token Type |
|---------------|--------------------|---------------|
| Y           | 'single quote'     | String        |
| Y           | "double quote"     | String        |
| Y           | 123                | Number        |
| Y           | 123.456            | Number        |
| Y           | =                  | Assign        |
| N           | +                  | Assign        |
| Y           | let                | Identifier    |
| N           | const              | Identifier    |
