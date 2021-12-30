# SDB - SignatureDB

Distributed, version controlled, database with cryptographically verifiable storage, queries
and, results that supports both a SQL and DataFrame query API.

Think [`git`](https://git-scm.com) for [`postgres`](https://www.postgresql.org) or [`pandas`](https://pandas.pydata.org).

```bash
$ isdb -d app_dev -U sdb
isdb (0.0.1)
Type "help" for help.

app_dev=# VERIFY HEAD
app_dev-# SELECT id, product_code, value, email
app_dev-# FROM orders;
 id | product_code | value | email
----+--------------+-------+-------
(0 rows)

SHA: 6f902734101b5963367f394aa0def3f3d50f760e
app_dev=# \q
---
$ sdb log -2
commit 6f902734101b5963367f394aa0def3f3d50f760e (test-orders, origin/main, origin/HEAD, main)
Author: alex <alex+git@fremantle.io>
Date:   Thu Dec 30 10:17:26 2021 -0800

    Initial commit
---
$ isdb -d app_dev -U sdb
isdb (0.0.1)
Type "help" for help.

app_dev=# MESSAGE 'First order!'
app_dev-# INSERT INTO orders (product_code, value, email)
app_dev-# VALUES ('abc123', 10.0, 'alex@fremantle.io');
INSERT 0 1

SHA: e427da4413bed914dfece023ed171ab73bb20c0b
app_dev=# VERIFY HEAD
app_dev-# SELECT id, product_code, value, email
app_dev-# FROM (orders BEFORE COMMIT HEAD);
 id | product_code | value |       email
----+--------------+-------+-------------------
 1  | abc123       | 10.0  | alex@fremantle.io
(1 row)

SHA: e427da4413bed914dfece023ed171ab73bb20c0b
app_dev=# \q
---
$ sdb log -2
commit e427da4413bed914dfece023ed171ab73bb20c0b (HEAD -> test-orders)
Author: alex <alex+git@fremantle.io>
Date:   Thu Dec 30 12:43:06 2021 -0800

    First order!

commit 6f902734101b5963367f394aa0def3f3d50f760e (test-orders, origin/main, origin/HEAD, main)
Author: alex <alex+git@fremantle.io>
Date:   Thu Dec 30 10:17:26 2021 -0800

    Initial commit
---
$
```

## What Problem Does This Solve?

`signaturedb` was conceived to solve 2 main problems:

1. Blockchain state bloat
2. Improve the web3 developer experience

Although `signaturedb` is inspired by problems within the blockchain space it is not a blockchain. It has
more in common with a regular, boring old SQL database like `postgres`. We believe that the demand for
verifiable data will grow if it is cheap, performant and has familiar developer ergonomics.

We strive to make `signaturedb` the go-to solution for any `OLTP` or `OLAP` workload that requires
application data verification all the way from small to big data. e.g.

- L1/L2 Blockchains
- Web3 Dapps with & without a blockchain
- [Addressing Audit Log Storage for U.S. Federal Government Customers](https://cloudblogs.microsoft.com/industry-blog/microsoft-in-business/government/2021/04/14/addressing-audit-log-storage-for-u-s-federal-government-customers/)
- [HIPAA Audit Trail and Audit Log](https://compliancy-group.com/hipaa-audit-log-requirements/)
- [Database backup and restore verification](https://docs.oracle.com/cd/B19306_01/backup.102/b14192/strategy004.htm)
- [Supply chain provenance](https://hbr.org/2010/10/the-transparent-supply-chain)
- Any database backed application that wants to verify data integrity

### 1. Blockchain State Bloat

From the earliest days of Bitcoin there have been [contentious opinions](https://blog.bitmex.com/the-blocksize-war-chapter-1-first-strike/) on 
the throughput, storage and verification of on-chain data. We believe that to realize the vision of web3
at global scale, to distribute control of data back to the user, we need solutions that operate at big
data scale (terrabyte/petabyte/exabyte etc...) but consider cost as a serious engineering challenge to maintain
decentralization. There have been recent advancements in the big data field around [clever and efficient data storage techniques](https://databricks.com/glossary/what-is-parquet), 
processing and interoperability. Many of these revolve around the [Apache Arrow](https://arrow.apache.org/)
project which `"provides an API for data"` and easily allows developers to [process large datasets efficiently using columnar storage techniques](https://towardsdatascience.com/apache-arrow-read-dataframe-with-zero-memory-69634092b1a)
with the power of [SIMD](https://en.wikipedia.org/wiki/Single_instruction,_multiple_data) on modern CPU's & GPU's.

For additional backround on the problems that arise in blockchain state managment the following resources may be helpful:

* https://eprint.iacr.org/2021/183
* https://medium.com/nervosnetwork/state-explosion-and-the-tragedy-of-the-blockchain-commons-1fbd4837e859
* https://medium.com/zulurepublic/ztx-explained-pt2-9eb7bac73656

### 2. Web3 Developer Experience

The Dapp development workflow has improved considerably in the few short years of its existence.

`signaturedb` continues to push improvements to the daily workflows of developers building `web3`
applications. It brings the convenience and stability of familiar battle tested tools like
`postgres` and `git` to enable fast, verifiable, collaboration on shared data stores.

## !!WARNING!!

`signaturedb` is currently a research project with many unsolved problems and no estimated
timeline for solving them. We have created an [outline](#status) of features and will update them as we progress.

## Getting Started

* [CLI](./docs/cli.md)
* [REPL](./docs/repl.md)
* [Query Statements](./docs/query_statements.md)
* [DDL](./docs/ddl.md)
* [Internals](./docs/internals.md)

## Status

### CLI

* [ ] `help`
* [ ] `createdb`
* [ ] `dropdb`

### REPL

* [ ] `\l`
* [ ] `\dn`
* [ ] `\dt`
* [ ] `\d`
* [ ] `\d relation`
* [ ] `\e`

### Query Statements

* [ ] `INSERT`
* [ ] `SELECT`
* [ ] `UPDATE`
* [ ] `DELETE`

### DDL

* [ ] `CREATE TABLE`
* [ ] `ALTER TABLE`
* [ ] `DROP TABLE`

## Authors

- Alex Kwiatkowski - alex+git@fremantle.io

## License

`signaturedb` is released under the [MIT license](./LICENSE)
