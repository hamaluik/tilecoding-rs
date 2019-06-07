## 2019-06-07, Version v0.3.0
### Commits
- [[`ee6beaafe3`](https://github.com/hamaluik/tilecoding-rs/commit/ee6beaafe33ad575614feda990441c5e5ec869cc)] removed mut reference requirements for readonly access to the IHT (Kenton Hamaluik)

### Stats
```diff
 Cargo.toml |  2 +-
 src/lib.rs | 12 ++++--------
 2 files changed, 5 insertions(+), 9 deletions(-)
```

## 2019-05-16, Version v0.2.0
### Commits
- [[`50f21f85ac`](https://github.com/hamaluik/tilecoding-rs/commit/50f21f85accd8e81dfc0c4715f539c7e4f7a1cc6)] simplified readme (Kenton Hamaluik)
- [[`fbd6a84f2f`](https://github.com/hamaluik/tilecoding-rs/commit/fbd6a84f2f2ee232eb9029d55f357b2742d8c02b)] added wrapping versions (Kenton Hamaluik)
- [[`68019b8c03`](https://github.com/hamaluik/tilecoding-rs/commit/68019b8c03ecafafce968fcba973225782d16789)] added a readonly mode to the IHT (Kenton Hamaluik)
- [[`2bd5bc0718`](https://github.com/hamaluik/tilecoding-rs/commit/2bd5bc07188decbdc7434e6002a373728c8bd546)] added changelog (Kenton Hamaluik)

### Stats
```diff
 CHANGELOG.md |  42 ++++++++++-
 Cargo.toml   |   4 +-
 README.md    |  22 +-----
 src/lib.rs   | 263 ++++++++++++++++++++++++++++++++++++++++++++++++++++--------
 4 files changed, 274 insertions(+), 57 deletions(-)
```

## 2019-05-16, Version v0.1.2
### Commits
- [[`e7b8a3aea7`](https://github.com/hamaluik/tilecoding-rs/commit/e7b8a3aea7cb7b066bbf2c7ee0a05f18de3520ee)] fixed index bug when IHT is full (Kenton Hamaluik)
- [[`98ddaaaeb2`](https://github.com/hamaluik/tilecoding-rs/commit/98ddaaaeb242b1e29ff83f209e6025f091de70e5)] Merge branch 'master' of github.com:hamaluik/tilecoding-rs (Kenton Hamaluik)

### Stats
```diff
 Cargo.toml |  2 +-
 src/lib.rs | 16 +++++++++++++++-
 2 files changed, 16 insertions(+), 2 deletions(-)
```

## 2019-05-16, Version v0.1.1
### Commits
- [[`78fb10f0b2`](https://github.com/hamaluik/tilecoding-rs/commit/78fb10f0b2f3dd36c13d378aefea2c3c5da113fa)] fixed docs badge (Kenton Hamaluik)
- [[`f0b51b97e2`](https://github.com/hamaluik/tilecoding-rs/commit/f0b51b97e237faed382d330ad344c960362b3d6d)] fixed cargo keywords (Kenton Hamaluik)
- [[`f854e6a58c`](https://github.com/hamaluik/tilecoding-rs/commit/f854e6a58c142858ad87f70e3370f1ff26e1fb5e)] added documentation (Kenton Hamaluik)
- [[`1b1ab6b60b`](https://github.com/hamaluik/tilecoding-rs/commit/1b1ab6b60b3771e6b3332964759b63e7ab4e5f82)] remove benchmarking, as it is nightly-only (Kenton Hamaluik)
- [[`d3e4a4e96c`](https://github.com/hamaluik/tilecoding-rs/commit/d3e4a4e96cec460b3fb9d71ace05967898c34aa2)] added basic travis config (Kenton Hamaluik)
- [[`7a7feb340c`](https://github.com/hamaluik/tilecoding-rs/commit/7a7feb340c6bd7d0de28ccd488719194d5e681af)] added badges to cargo and readme (Kenton Hamaluik)
- [[`d1545bf3f9`](https://github.com/hamaluik/tilecoding-rs/commit/d1545bf3f9cddf636a2a7d04900c0eaf8d4fb477)] added cargo metadata, fleshed out README, added license (Kenton Hamaluik)
- [[`f9e5a8a5ba`](https://github.com/hamaluik/tilecoding-rs/commit/f9e5a8a5ba46ce2dfc753913371d39471566f97c)] made ints optional (Kenton Hamaluik)
- [[`8afa193a7f`](https://github.com/hamaluik/tilecoding-rs/commit/8afa193a7fabca991d3ebdc6d6c4f08e3816c2cd)] added ints parameter (Kenton Hamaluik)
- [[`0faa6a7234`](https://github.com/hamaluik/tilecoding-rs/commit/0faa6a7234d73db340a44a41d5bd580e3efea0c5)] some cleanup to more closely match the source (Kenton Hamaluik)
- [[`eb5a44dcbf`](https://github.com/hamaluik/tilecoding-rs/commit/eb5a44dcbfb97dafefa56367d160d3876cf5336f)] made standalone function public 🙃 (Kenton Hamaluik)
- [[`69bf33b553`](https://github.com/hamaluik/tilecoding-rs/commit/69bf33b55373865ea284a1f4f681d094626b820a)] formatted (Kenton Hamaluik)
- [[`a6f6cc2b8c`](https://github.com/hamaluik/tilecoding-rs/commit/a6f6cc2b8c432e9865ed96258bd494f546ad6c25)] added version without IHT (Kenton Hamaluik)
- [[`8ded47ed4a`](https://github.com/hamaluik/tilecoding-rs/commit/8ded47ed4a216c8fd49f16d5d524b1acf887833e)] moved basehash out of IHT (Kenton Hamaluik)
- [[`1535914ee6`](https://github.com/hamaluik/tilecoding-rs/commit/1535914ee63fce271f5b950f2baf213af2510421)] use native rust hash with tiles3 version (Kenton Hamaluik)
- [[`487806fcce`](https://github.com/hamaluik/tilecoding-rs/commit/487806fcced01b7b36125856c1ff456240a178e5)] 🚀 (Kenton Hamaluik)

### Stats
```diff
 .gitignore     |   3 +-
 .travis.yml    |   9 ++-
 Cargo.toml     |  17 +++-
 LICENSE-APACHE | 201 ++++++++++++++++++++++++++++++++++++++-
 LICENSE-MIT    |  25 +++++-
 README.md      |  65 ++++++++++++-
 src/lib.rs     | 304 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++-
 7 files changed, 624 insertions(+)
```
