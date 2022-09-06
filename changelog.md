<a name="v5.14.0"></a>

## 5.14.1 (2022-09-06)

<csr-id-9f8bb80c4b45e9360f7ba6e9b21710b3a74e9b34/>

### BREAKING

- upgrade `mio` from 0.6 to 0.8.

### Chore

 - <csr-id-9f8bb80c4b45e9360f7ba6e9b21710b3a74e9b34/> bump mio dependency

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 613 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - prepare changelog for release ([`2152092`](https://github.com/Byron/yup-hyper-mock/commit/2152092f78897fea82b7df6ceeb382ba40b81209))
    - bump mio dependency ([`9f8bb80`](https://github.com/Byron/yup-hyper-mock/commit/9f8bb80c4b45e9360f7ba6e9b21710b3a74e9b34))
</details>

## v5.14.0 (2020-12-30)

<csr-id-be8c9e3b78d9efd0713737e9cbbb15f421ee6eb1/>

Upgrade to Hyper 0.14 and Tokio 1.0, which will break any dependencies in Tokio 0.2
environments.

<a name="v3.12.0"></a>

### Chore

 - <csr-id-be8c9e3b78d9efd0713737e9cbbb15f421ee6eb1/> Update to Hyper 0.14 and Tokio 1.0

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 262 calendar days.
 - 333 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update to Hyper 0.14 and Tokio 1.0 ([`be8c9e3`](https://github.com/Byron/yup-hyper-mock/commit/be8c9e3b78d9efd0713737e9cbbb15f421ee6eb1))
    - remove bench run, as there are none and release builds take time ([`a34fbcf`](https://github.com/Byron/yup-hyper-mock/commit/a34fbcf960908dacc9b1e26022c6ceffe8099144))
    - Apply cargo-fmt ([`e5c123f`](https://github.com/Byron/yup-hyper-mock/commit/e5c123f848d2cbd9c37ca723077c33ed74734d6d))
    - update badge ([`23beab9`](https://github.com/Byron/yup-hyper-mock/commit/23beab9b992e3e1ca0990551555a5fdd5575387c))
    - add github actions; remove travis :*( ([`a7310ba`](https://github.com/Byron/yup-hyper-mock/commit/a7310bae60c1b17b3565b015580d269f84e7efe9))
</details>

## v4.0.0 (2020-02-01)

### New Features

 - <csr-id-abb96132641328eeca7b3a3f10ea574bed5dc9ab/> update to `async`/`await`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 344 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump major version ([`c56f023`](https://github.com/Byron/yup-hyper-mock/commit/c56f0237f8d71e25739ef01bbdc0f2fe133ad4cc))
    - Optimize package size ([`a792dc5`](https://github.com/Byron/yup-hyper-mock/commit/a792dc5bbba5183887c08d332258591debafe884))
    - Remove clog - not a fan of subject-line rules anymore ([`7d7dcba`](https://github.com/Byron/yup-hyper-mock/commit/7d7dcbac1df89b3dee74760f6c8386da626acc8c))
    - Simplify .travis; remove editor settings ([`06da158`](https://github.com/Byron/yup-hyper-mock/commit/06da158e891da927c7658353445b607b629fbc8b))
    - update to `async`/`await` ([`abb9613`](https://github.com/Byron/yup-hyper-mock/commit/abb96132641328eeca7b3a3f10ea574bed5dc9ab))
</details>

## v3.15.0 (2019-02-21)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release.
 - 62 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump minor version: we don't support Rust 2015 anymore ([`819c888`](https://github.com/Byron/yup-hyper-mock/commit/819c888354ebd761c7b851c090ff52332cbc4798))
    - Merge pull request #19 from TheBiggerGuy/edition_2018 ([`d4981d9`](https://github.com/Byron/yup-hyper-mock/commit/d4981d9cecb343cfecdb339f1b7fc8a1491c401c))
    - Make code more 2018 idiomatic ([`ddeb3f2`](https://github.com/Byron/yup-hyper-mock/commit/ddeb3f26832d0a1f73c7e67e4dccdd79cc5274d7))
    - Convert to edition=2018 ([`11a20e7`](https://github.com/Byron/yup-hyper-mock/commit/11a20e7bd2ec1ecfc2bcfa0bdf1b57253ff40b14))
    - Set edition=2015 ([`5d458c6`](https://github.com/Byron/yup-hyper-mock/commit/5d458c6b99bfba1d89df579cd26fdc2b0f78b1a1))
</details>

## v3.14.0 (2018-12-21)

### Bug Fixes

 - <csr-id-f2acccd03f1e96306c31105325f4b6a83c741924/> set clear_write_ready
   When hyper does not flush prior to a yup_hyper_mock write, the library
   should set `clear_write_ready` to signal a subsequent write.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 61 calendar days.
 - 138 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version; include fix for polling logic ([`df83f73`](https://github.com/Byron/yup-hyper-mock/commit/df83f735c9e42ec8d19feb3b466c1964301fd00c))
    - set clear_write_ready ([`f2acccd`](https://github.com/Byron/yup-hyper-mock/commit/f2acccd03f1e96306c31105325f4b6a83c741924))
    - Fix GFM ([`a6dfd2b`](https://github.com/Byron/yup-hyper-mock/commit/a6dfd2bc6017174866818ce20d37321b1d0a7afe))
</details>

## v3.13.0 (2018-08-05)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release over the course of 2 calendar days.
 - 55 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump version for: Implement clone for mock ([`73fa137`](https://github.com/Byron/yup-hyper-mock/commit/73fa137837f174eb771b136d6736a95f7f153103))
    - Merge pull request #15 from rolftimmermans/master ([`f681008`](https://github.com/Byron/yup-hyper-mock/commit/f6810086ba934ee2401acc45001eea1e1478a93b))
    - Add Clone. ([`5b40a89`](https://github.com/Byron/yup-hyper-mock/commit/5b40a898f6e4e11f26492af76e3a4c9423ec9d50))
</details>

## v3.12.0 (2018-06-10)

<csr-id-3c129bc41eee41dfa40f6584ec950de230083451/>
<csr-id-3a9d8fc47d07c71d20311b07aea331e3ae8e38e1/>
<csr-id-58b5733b021160c0637687612fe61035d3dd1791/>
<csr-id-b85385f7a69766a2271df90368d06266dc997a63/>

Upgrade to hyper v0.12 - this might break dependencies, thus the major version jump.
The minor version now tracks the hyper version - it's a feature :D.

<a name="v2.0.0"></a>

### Test

 - <csr-id-3c129bc41eee41dfa40f6584ec950de230083451/> Add testing to Travis builds
   Travis builds didn't previously run the test suite, and now they do

### Documentation

 - <csr-id-97c5521d5a0be9ce73332c4dfa186b7d9255ffd3/> update import syntax
 - <csr-id-2477f4e45957827a489b5d1cc73491af5d38e1ba/> fix example

### Chore

 - <csr-id-3a9d8fc47d07c71d20311b07aea331e3ae8e38e1/> Relicense under Apache/MIT

 - <csr-id-58b5733b021160c0637687612fe61035d3dd1791/> v2.0
 - <csr-id-b85385f7a69766a2271df90368d06266dc997a63/> Upgrade hyper to 0.10
   This solves dermesser/yup-oauth2#51 problems, where two different
   versions of openssl-sys are included, one of which is pulled in by this
   crate.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 17 commits contributed to the release over the course of 836 calendar days.
 - 899 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Bump version to 3.12.0 ([`464de3c`](https://github.com/Byron/yup-hyper-mock/commit/464de3cda2637ec4e318c4230e3ece3d0f22632a))
    - Merge pull request #12 from palfrey/hyper-0.11 ([`dcfc643`](https://github.com/Byron/yup-hyper-mock/commit/dcfc643191664fe001103beffb8f5c9fb6517546))
    - Merge pull request #13 from palfrey/test-in-travis ([`c5c5783`](https://github.com/Byron/yup-hyper-mock/commit/c5c5783d7a9946dd07952bcaefe529029fd5e3e6))
    - Add testing to Travis builds ([`3c129bc`](https://github.com/Byron/yup-hyper-mock/commit/3c129bc41eee41dfa40f6584ec950de230083451))
    - Split out the mock streams ([`a474362`](https://github.com/Byron/yup-hyper-mock/commit/a47436257c78032c14c4b9bae68ba95ba0f1f2a6))
    - Functioning tests with Hyper 0.12 ([`dad576a`](https://github.com/Byron/yup-hyper-mock/commit/dad576aefba30959d0a5bc46f5541e97b15fb060))
    - First work that semi-works with Tokio ([`1077ca8`](https://github.com/Byron/yup-hyper-mock/commit/1077ca8d119c6da1c45620086378f506b46aef20))
    - Initial hyper 0.11 version that at least compiles ([`deb5186`](https://github.com/Byron/yup-hyper-mock/commit/deb518683b2c765bbf6d7e5ea859f49373937fce))
    - Add crates version badge ([`0780f30`](https://github.com/Byron/yup-hyper-mock/commit/0780f300c134693cbbf8f963554c49f2b6304552))
    - Relicense under Apache/MIT ([`3a9d8fc`](https://github.com/Byron/yup-hyper-mock/commit/3a9d8fc47d07c71d20311b07aea331e3ae8e38e1))
    - v2.0 ([`58b5733`](https://github.com/Byron/yup-hyper-mock/commit/58b5733b021160c0637687612fe61035d3dd1791))
    - Merge pull request #10 from dermesser/upgrade-hyper ([`25431ca`](https://github.com/Byron/yup-hyper-mock/commit/25431cafef83cc656bcb5a71f231ca139b6cc87d))
    - Upgrade hyper to 0.10 ([`b85385f`](https://github.com/Byron/yup-hyper-mock/commit/b85385f7a69766a2271df90368d06266dc997a63))
    - Merge pull request #8 from Ryman/patch-1 ([`1856d16`](https://github.com/Byron/yup-hyper-mock/commit/1856d1614ae67a30c0e0d49dbb624bfa40567f19))
    - update import syntax ([`97c5521`](https://github.com/Byron/yup-hyper-mock/commit/97c5521d5a0be9ce73332c4dfa186b7d9255ffd3))
    - Merge pull request #7 from rsolomo/patch-1 ([`cd0bfa8`](https://github.com/Byron/yup-hyper-mock/commit/cd0bfa8b50d60a711340a1999cc81cdb7a1cf5ee))
    - fix example ([`2477f4e`](https://github.com/Byron/yup-hyper-mock/commit/2477f4e45957827a489b5d1cc73491af5d38e1ba))
</details>

## v2.0.0 (2017-01-31)

Upgrade to hyper v0.10 - this might break dependencies, thus the major version jump.


<a name="50199133"></a>

## 50199133 (2015-06-26)

### Bug Fixes

* **hyper-up**  work with hyper 0.6.0 ([50199133](https://github.com/Byron/yup-hyper-mock/commit/501991335dde3c88e845277e3772708cc1f5eef0))

<csr-unknown>
<csr-unknown>
<a name="v1.2.0"></a><csr-unknown/>
<csr-unknown/>

## v1.3.2 (2015-12-24)

<csr-id-8f09de7b03ebea21b37d868f9da99ed7845df542/>

### Chore

 - <csr-id-8f09de7b03ebea21b37d868f9da99ed7845df542/> v1.3.2
   Avoid wildcards in dependencies

### Bug Fixes

 - <csr-id-a2766102527344f940c0e0aa51b5fee7a17b54c5/> works with hyper 0.7
   The NetworkStream trait added two additional required methods. They are
   not implemented for both TeeStream and MockStream.
 - <csr-id-682f6e6311f0068690ec8eafcb11d12d3d679bad/> client doesn't need mutability

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 138 calendar days.
 - 181 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - v1.3.2 ([`8f09de7`](https://github.com/Byron/yup-hyper-mock/commit/8f09de7b03ebea21b37d868f9da99ed7845df542))
    - Merge pull request #4 from jwilm/hyper-0.7-compat ([`382a16a`](https://github.com/Byron/yup-hyper-mock/commit/382a16a5000884f5db392c8e2c57d48824b706ac))
    - works with hyper 0.7 ([`a276610`](https://github.com/Byron/yup-hyper-mock/commit/a2766102527344f940c0e0aa51b5fee7a17b54c5))
    - client doesn't need mutability ([`682f6e6`](https://github.com/Byron/yup-hyper-mock/commit/682f6e6311f0068690ec8eafcb11d12d3d679bad))
</details>

## v1.3.0 (2015-06-26)

<csr-id-e81c7a3001415f3af7d86ea49bd6d9d74631ff61/>

### Chore

 - <csr-id-e81c7a3001415f3af7d86ea49bd6d9d74631ff61/> v1.3.0

### Bug Fixes

 - <csr-id-501991335dde3c88e845277e3772708cc1f5eef0/> work with hyper 0.6.0

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 7 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - v1.3.0 ([`e81c7a3`](https://github.com/Byron/yup-hyper-mock/commit/e81c7a3001415f3af7d86ea49bd6d9d74631ff61))
    - work with hyper 0.6.0 ([`5019913`](https://github.com/Byron/yup-hyper-mock/commit/501991335dde3c88e845277e3772708cc1f5eef0))
</details>

## v1.2.0 (2015-06-18)

<csr-id-d0a72f96a7e1507c77af2daa183dfe2a48a4de27/>
<csr-id-f59454a8b70a1f262dea55b387fe0da1f8292886/>

#### Features

* **lib**  
  * added HostToReplyConnector type ([66763316](https://github.com/Byron/yup-hyper-mock/commit/667633168574c731ec0aa1f0266aa3be79e5d8d6))
  * added SequentialConnector type ([553c9dcb](https://github.com/Byron/yup-hyper-mock/commit/553c9dcb7c7f156e706c36ff7029464b8950df4d))

<a name="1.0.1"></a>

### Refactor

 - <csr-id-d0a72f96a7e1507c77af2daa183dfe2a48a4de27/> no `RefCell` in SequentialConnector
   It's not actually required, as the vector is not changed within an
   immutable context anymore.

### New Features

 - <csr-id-667633168574c731ec0aa1f0266aa3be79e5d8d6/> added HostToReplyConnector type
   * a type allowing a singular reply per host url. It can be useful
   if you cannot use the provided convenience macro, `mock_connector`.
* macro was adjusted to use that type instead, making it far more
     readable.

### Chore

 - <csr-id-f59454a8b70a1f262dea55b387fe0da1f8292886/> v1.2.0

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - v1.2.0 ([`f59454a`](https://github.com/Byron/yup-hyper-mock/commit/f59454a8b70a1f262dea55b387fe0da1f8292886))
    - no `RefCell` in SequentialConnector ([`d0a72f9`](https://github.com/Byron/yup-hyper-mock/commit/d0a72f96a7e1507c77af2daa183dfe2a48a4de27))
    - added HostToReplyConnector type ([`6676331`](https://github.com/Byron/yup-hyper-mock/commit/667633168574c731ec0aa1f0266aa3be79e5d8d6))
</details>

## v1.1.0 (2015-06-18)

<csr-id-0f5a4ae0db8ad87bef78c38073eec3aaeb9d3ead/>
<csr-id-c95249289b4407ba8ac9a7fb7f7ad89b6d126bad/>

### Chore

 - <csr-id-0f5a4ae0db8ad87bef78c38073eec3aaeb9d3ead/> v1.1.0

### New Features

 - <csr-id-553c9dcb7c7f156e706c36ff7029464b8950df4d/> added SequentialConnector type
   It allows to implement something similar to the
   `mock_connector_in_order` macro, but can be used in cases where macros
   don't work. An example of this is if an exported macro is used
   in an `!include` macro.

### Refactor

 - <csr-id-c95249289b4407ba8ac9a7fb7f7ad89b6d126bad/> use SequentialConnector in macro
   Previously the macro would implement its own, hard-to-read type,
   now it just creates a NewType from the pre-implemented custom type.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 7 days passed between releases.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - v1.1.0 ([`0f5a4ae`](https://github.com/Byron/yup-hyper-mock/commit/0f5a4ae0db8ad87bef78c38073eec3aaeb9d3ead))
    - use SequentialConnector in macro ([`c952492`](https://github.com/Byron/yup-hyper-mock/commit/c95249289b4407ba8ac9a7fb7f7ad89b6d126bad))
    - added SequentialConnector type ([`553c9dc`](https://github.com/Byron/yup-hyper-mock/commit/553c9dcb7c7f156e706c36ff7029464b8950df4d))
</details>

## 1.0.1 (2015-06-11)

<csr-id-9aac86a680e57254767d8c7eb1b7318f6923c4ec/>
<csr-id-7d669e40073913394660546ee59a2ef5df81c9c1/>
<csr-id-c38a54f877c6460fa36c2d8d71142237c9dec541/>

### Other

 - <csr-id-9aac86a680e57254767d8c7eb1b7318f6923c4ec/> added integration tests
   This shows that the macros are not actually working anymore due to
   signature changes.

### Chore

 - <csr-id-7d669e40073913394660546ee59a2ef5df81c9c1/> v1.0.1
 - <csr-id-c38a54f877c6460fa36c2d8d71142237c9dec541/> set to rust-stable
   This is the one we should keep running on.

### Bug Fixes

* **macros**  adjust to changed hyper signatures ([be814d00](https://github.com/Byron/yup-hyper-mock/commit/be814d0087f791cb0597fce19e6093add41502c4))
 - <csr-id-be814d0087f791cb0597fce19e6093add41502c4/> adjust to changed hyper signatures
   As connect now takes a `&self` only, we had to use cells to mutate our
   state in the case of the sequential mock connector.
   
   It's a bit messy, and ain't pretty, but it now works at least.
   
   Thanks to the tests, it should stay that way as well.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 23 calendar days.
 - 23 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - v1.0.1 ([`7d669e4`](https://github.com/Byron/yup-hyper-mock/commit/7d669e40073913394660546ee59a2ef5df81c9c1))
    - adjust to changed hyper signatures ([`be814d0`](https://github.com/Byron/yup-hyper-mock/commit/be814d0087f791cb0597fce19e6093add41502c4))
    - added integration tests ([`9aac86a`](https://github.com/Byron/yup-hyper-mock/commit/9aac86a680e57254767d8c7eb1b7318f6923c4ec))
    - set to rust-stable ([`c38a54f`](https://github.com/Byron/yup-hyper-mock/commit/c38a54f877c6460fa36c2d8d71142237c9dec541))
    - Merge branch 'next' ([`4dda44d`](https://github.com/Byron/yup-hyper-mock/commit/4dda44dde36f71782d3dd7b040017c0181fba94f))
</details>

## 1.0.0 (2015-05-18)

<csr-id-cc72c7a560ce849a12e6401942f4ba166e5c4b9b/>
<csr-id-ee56de4dead136b3ca5a3eda6ca7057f9074e261/>
<csr-id-555a22f9786d43ea03d475429c45ef501eda74ea/>

### Chore

 - <csr-id-cc72c7a560ce849a12e6401942f4ba166e5c4b9b/> 1.0 stable
   We build on stable, and work with hyper 1.0
 - <csr-id-ee56de4dead136b3ca5a3eda6ca7057f9074e261/> assure we get the right hyper
   Need to be precise here, must match all projects using yup-hyper-mock
   as well. This should be fixed asap, depends on the next hyper release.
 - <csr-id-555a22f9786d43ea03d475429c45ef501eda74ea/> reset to previous version of hyper
   That way, we can still build it from git directly, as long as hyper
   0.4.X is not out.
   
   It kind of shows that proper version management on the maintainer side
   is critical to allow these kinds of applications, which would mean
   you set your version to the next one once the current one has been
   released.

### Bug Fixes

* **hyperup**  adapt to hyper >v0.4.0 ([388f9a0e](https://github.com/Byron/yup-hyper-mock/commit/388f9a0e3c7ed057dabe30aa209e0c05039c2274))
 - <csr-id-388f9a0e3c7ed057dabe30aa209e0c05039c2274/> adapt to hyper >v0.4.0
   * adjust to NetworkConnector trait changes

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release over the course of 6 calendar days.
 - 9 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - 1.0 stable ([`cc72c7a`](https://github.com/Byron/yup-hyper-mock/commit/cc72c7a560ce849a12e6401942f4ba166e5c4b9b))
    - assure we get the right hyper ([`ee56de4`](https://github.com/Byron/yup-hyper-mock/commit/ee56de4dead136b3ca5a3eda6ca7057f9074e261))
    - reset to previous version of hyper ([`555a22f`](https://github.com/Byron/yup-hyper-mock/commit/555a22f9786d43ea03d475429c45ef501eda74ea))
    - adapt to hyper >v0.4.0 ([`388f9a0`](https://github.com/Byron/yup-hyper-mock/commit/388f9a0e3c7ed057dabe30aa209e0c05039c2274))
</details>

## v0.1.8 (2015-05-08)

<csr-id-6100662f37fe1bfb7dfb89ce8322eb14b427dfb2/>
<csr-id-3c1e3e89a22ea48307286550a92abc76a5eb7212/>

#### Features

* **compatibility**  Update to reflect Hyper v0.4 ([5078314f](https://github.com/Byron/yup-hyper-mock/commit/5078314f3ef33381fce92317a9f42d31f0067e7e))

### New Features

 - <csr-id-5078314f3ef33381fce92317a9f42d31f0067e7e/> Update to reflect Hyper v0.4
   Breaking Change was added to Hyper in this commit:
   https://github.com/hyperium/hyper/commit/972b
   
   This commit hopes to mirror those changes into this copy of the mock
   functionality.

### Chore

 - <csr-id-6100662f37fe1bfb7dfb89ce8322eb14b427dfb2/> update for v0.1.8
   Also improved clog configuration
 - <csr-id-3c1e3e89a22ea48307286550a92abc76a5eb7212/> switch to travis-cargo

### Bug Fixes

* **macro**  use hyper::Result ([ae019daf](https://github.com/Byron/yup-hyper-mock/commit/ae019daf13181a570570500c17b58dbd54c8f55e))
 - <csr-id-ae019daf13181a570570500c17b58dbd54c8f55e/> use hyper::Result
   Previously it used $crate::Result, which doesn't actually work.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 1 calendar day.
 - 3 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - update for v0.1.8 ([`6100662`](https://github.com/Byron/yup-hyper-mock/commit/6100662f37fe1bfb7dfb89ce8322eb14b427dfb2))
    - use hyper::Result ([`ae019da`](https://github.com/Byron/yup-hyper-mock/commit/ae019daf13181a570570500c17b58dbd54c8f55e))
    - Merge branch 'talevy-update-0.4' ([`a534fba`](https://github.com/Byron/yup-hyper-mock/commit/a534fbae3ddd84e22a674510a6167e2bc07ab5de))
    - Update to reflect Hyper v0.4 ([`5078314`](https://github.com/Byron/yup-hyper-mock/commit/5078314f3ef33381fce92317a9f42d31f0067e7e))
    - switch to travis-cargo ([`3c1e3e8`](https://github.com/Byron/yup-hyper-mock/commit/3c1e3e89a22ea48307286550a92abc76a5eb7212))
</details>

## v0.1.6 (2015-05-04)

<csr-id-24f20997fbea44f706cc41a31a20391b82fa8549/>
<csr-id-8a218b5d1159b23c87082ce81b0cbe3220a673d3/>
<csr-id-e624b3b7fcf54099209e02c25114298839e628ca/>

### Bug Fixes

* **macro**  use $crate variable instead of ::mock ([b520fa77](https://github.com/Byron/yup-hyper-mock/commit/b520fa77f44262598e92149f8fd995b0543b7739), closes [#2](https://github.com/Byron/yup-hyper-mock/issues/2))
 - <csr-id-b520fa77f44262598e92149f8fd995b0543b7739/> use $crate variable instead of ::mock
   That way, it will actually work if the crate-name of `yup-hyper-mock`
   is not changed to mock.
* **clog**  added changelog, managed by clog ([1a1b3fa3](https://github.com/Byron/yup-hyper-mock/commit/1a1b3fa34d1c9c919a38a3a2a392422cd71c8db8))

### New Features

 - <csr-id-1a1b3fa34d1c9c919a38a3a2a392422cd71c8db8/> added changelog, managed by clog
   * also incremented version

### Chore

 - <csr-id-24f20997fbea44f706cc41a31a20391b82fa8549/> update for v0.1.6
 - <csr-id-8a218b5d1159b23c87082ce81b0cbe3220a673d3/> single-line to multi-line
   That way, each travis-cargo incocation will show up individually
   in the travis log.
   
   Tests are disabled just because there are none, as well as coverage.
   [skip ci]
 - <csr-id-e624b3b7fcf54099209e02c25114298839e628ca/> added sublime-linter rust support
   [skip ci]

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 2 calendar days.
 - 25 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - update for v0.1.6 ([`24f2099`](https://github.com/Byron/yup-hyper-mock/commit/24f20997fbea44f706cc41a31a20391b82fa8549))
    - added changelog, managed by clog ([`1a1b3fa`](https://github.com/Byron/yup-hyper-mock/commit/1a1b3fa34d1c9c919a38a3a2a392422cd71c8db8))
    - single-line to multi-line ([`8a218b5`](https://github.com/Byron/yup-hyper-mock/commit/8a218b5d1159b23c87082ce81b0cbe3220a673d3))
    - use $crate variable instead of ::mock ([`b520fa7`](https://github.com/Byron/yup-hyper-mock/commit/b520fa77f44262598e92149f8fd995b0543b7739))
    - added sublime-linter rust support ([`e624b3b`](https://github.com/Byron/yup-hyper-mock/commit/e624b3b7fcf54099209e02c25114298839e628ca))
</details>

<csr-unknown>
Features<a name="v0.1.5"></a><csr-unknown/>

## v0.1.5 (2015-04-08)

### Bug Fixes

* **version-up**  v0.1.5 (hyper adjustments) ([65b3baa0](https://github.com/Byron/yup-hyper-mock/commit/65b3baa0b7ffe05cf1047010d6de3273f3057ffd))
 - <csr-id-65b3baa0b7ffe05cf1047010d6de3273f3057ffd/> v0.1.5 (hyper adjustments)
   * Deal with changed signature of `NetworkConnector`

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - v0.1.5 (hyper adjustments) ([`65b3baa`](https://github.com/Byron/yup-hyper-mock/commit/65b3baa0b7ffe05cf1047010d6de3273f3057ffd))
</details>

## v0.1.4 (2015-04-07)

### Bug Fixes

* **rustup**
  *  rustc (be9bd7c93 2015-04-05) ([6e8616a7](https://github.com/Byron/yup-hyper-mock/commit/6e8616a7180fec5d07ab85230fa7d54ee2ee3c97))
  *  remove old_io entirely ([d6c4943b](https://github.com/Byron/yup-hyper-mock/commit/d6c4943bd3e9f2029baa56e68e60b129f2afc9cb))
*  remove old_io entirely ([d6c4943b](https://github.com/Byron/yup-hyper-mock/commit/d6c4943bd3e9f2029baa56e68e60b129f2afc9cb))
 - <csr-id-c112b907de1d0a380947d22443466c5beed12ef0/> added license info
 - <csr-id-d6c4943bd3e9f2029baa56e68e60b129f2afc9cb/> remove old_io entirely
   Mock was also updated from the latest version in hyper.
   A good thing to do!
 - <csr-id-bbb0e73953c8931f7ed1ecfe85f7f3198bedeeb1/> work as well now (thanks, oauth2)
   There are no regression tests for macros, which caused them to never
   be tested actually.
 - <csr-id-6e8616a7180fec5d07ab85230fa7d54ee2ee3c97/> rustc (be9bd7c93 2015-04-05)
   * fix compile warnings
* **mock**  allow shared mock types ([981d8bfb](https://github.com/Byron/yup-hyper-mock/commit/981d8bfb408c63d7705d565f734207c8df567d2f))

### New Features

 - <csr-id-981d8bfb408c63d7705d565f734207c8df567d2f/> allow shared mock types
   Sharing only works across unit-tests, but it's better than nothing.
   Added note to docs as well to explain that.

### Documentation

 - <csr-id-ad83e7d6883e1e28d31d66b10fc92f33c4735edf/> guidelines added
   Just copied from yup-oauth2
 - <csr-id-a5584bc31384328ac79d6553bc057f261ebaf6b7/> added travis badge

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 39 calendar days.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - rustc (be9bd7c93 2015-04-05) ([`6e8616a`](https://github.com/Byron/yup-hyper-mock/commit/6e8616a7180fec5d07ab85230fa7d54ee2ee3c97))
    - work as well now (thanks, oauth2) ([`bbb0e73`](https://github.com/Byron/yup-hyper-mock/commit/bbb0e73953c8931f7ed1ecfe85f7f3198bedeeb1))
    - remove old_io entirely ([`d6c4943`](https://github.com/Byron/yup-hyper-mock/commit/d6c4943bd3e9f2029baa56e68e60b129f2afc9cb))
    - allow shared mock types ([`981d8bf`](https://github.com/Byron/yup-hyper-mock/commit/981d8bfb408c63d7705d565f734207c8df567d2f))
    - guidelines added ([`ad83e7d`](https://github.com/Byron/yup-hyper-mock/commit/ad83e7d6883e1e28d31d66b10fc92f33c4735edf))
    - added license info ([`c112b90`](https://github.com/Byron/yup-hyper-mock/commit/c112b907de1d0a380947d22443466c5beed12ef0))
    - added travis badge ([`a5584bc`](https://github.com/Byron/yup-hyper-mock/commit/a5584bc31384328ac79d6553bc057f261ebaf6b7))
    - Added documentation URL ([`6aafcbc`](https://github.com/Byron/yup-hyper-mock/commit/6aafcbc065b4d85adc55b90ae0999a5231907cf2))
    - Bumped version to 0.1 - it's actually usable ... ([`4e4c05b`](https://github.com/Byron/yup-hyper-mock/commit/4e4c05bd6fb00867c5c3226e7fdde6a7bca5fc41))
    - Added missing documentation ([`1ff21e0`](https://github.com/Byron/yup-hyper-mock/commit/1ff21e0686cb9bf7b85781ddc5b71736169be151))
    - Setup travis and sublime ([`44efaf9`](https://github.com/Byron/yup-hyper-mock/commit/44efaf936c00bbb88102e9af658e610b673a0a4a))
    - initial commit after removing it from yup/lib ([`aa284eb`](https://github.com/Byron/yup-hyper-mock/commit/aa284eb98e49e8d79fc9bed8d9e2146f1e634ecd))
</details>

<csr-unknown>
remove old_io entirely (https://github.com/Byron/yup-hyper-mock/commit/d6c4943bd3e9f2029baa56e68e60b129f2afc9cbd6c4943b)Features<csr-unknown/>

