# REPL - SignatureDB

An interactive terminal frontend that enables you to type queries, issue them to SDB and see the query results.

```bash
$ isdb -d app_dev -U sdb
isdb (0.0.1)
Type "help" for help.

app_dev=#
```

## Meta Commands

### List databases

```bash
app_dev=# \l
 List of databases
       Name
------------------
 app_dev
 app_test
(2 rows)
```

### List schemas

```bash
app_dev=# \dn
TODO...
```

### List relations

```bash
app_dev=# \d
           List of relations
 Schema |       Name      |   Type
--------+-----------------+----------
 public | accounts        | table
 public | orders          | table
 public | orders_id_seq   | sequence
 (3 rows)
```

### List a specified relation

```bash
app_dev=# \d orders
                                            Table "public.orders"
     Column      |              Type              | Collation | Nullable |               Default
-----------------+--------------------------------+-----------+----------+--------------------------------------
 id              | bigint                         |           | not null | nextval('orders_id_seq'::regclass)
 product_code    | character varying(255)         |           | not null |
 value           | numeric                        |           | not null |
 email           | character varying(255)         |           | not null |
 inserted_at     | timestamp(0) without time zone |           | not null |
 updated_at      | timestamp(0) without time zone |           | not null |
Indexes:
    "orders_pkey" PRIMARY KEY, btree (id)
Storage:
    Row
```
