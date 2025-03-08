namespace SqlCoderPoc;

public static class NorthwindPrompt
{
    public static string GetPrompt(string question)
    {
        return $$"""
                ### Instructions:
                Your task is to convert a question into a SQL query, given a Postgres database schema.
                Adhere to these rules:
                - **Deliberately go through the question and database schema word by word** to appropriately answer the question
                - **Use Table Aliases** to prevent ambiguity. For example, `SELECT table1.col1, table2.col1 FROM table1 JOIN table2 ON table1.id = table2.id`.
                - When creating a ratio, always cast the numerator as float

                ### Input:

                ### Task
                Generate a SQL query that answers the question `{{question}}`

                ### Database Schema
                This query will run on a database whose schema is represented in this string:
                
                CREATE TABLE categories (
                    category_id smallint NOT NULL,
                    category_name character varying(15) NOT NULL,
                    description text,
                    picture bytea
                );

                CREATE TABLE suppliers (
                    supplier_id smallint NOT NULL,
                    company_name character varying(40) NOT NULL,
                    contact_name character varying(30),
                    contact_title character varying(30),
                    address character varying(60),
                    city character varying(15),
                    region character varying(15),
                    postal_code character varying(10),
                    country character varying(15),
                    phone character varying(24),
                    fax character varying(24),
                    homepage text
                );

                CREATE TABLE products (
                    product_id smallint NOT NULL,
                    product_name character varying(40) NOT NULL,
                    supplier_id smallint,
                    category_id smallint,
                    quantity_per_unit character varying(20),
                    unit_price real,
                    units_in_stock smallint,
                    units_on_order smallint,
                    reorder_level smallint,
                    discontinued integer NOT NULL
                );

                -- products.supplier_id can be joined with suppliers.supplier_id
                -- products.category_id can be joined with categories.category_id
                
                ### Response:
                Based on your instructions, here is the SQL query I have generated to answer the question `{{question}}`:
                
                """;
    }
}
