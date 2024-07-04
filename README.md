# Flaky test detector

Find flaky tests by running tests a lot.

## Usage

```bash
$ flaky-test-detector --repeat 20 --test-command "yarn workspace @project/server test-one {}" src/lib/service/my_test_one.test.ts src/lib/service/my_test_two.test.ts
Running tests in file: src/lib/service/my_test_one.test.ts
Running tests in file: src/lib/service/my_test_two.test.ts
Finished running tests in file: src/lib/service/my_test_one.test.ts in 44077 ms
Finished running tests in file: src/lib/service/my_test_two.test.ts in 192631 ms
Test: src/lib/service/my_test_one.test.ts [avg runtime: 2262 ms]
Fail percentage: 30%
Test: src/lib/service/my_test_two.test.ts [avg runtime: 9944 ms]
Fail percentage: 0%

Analyzed 2 tests in 192632 ms
```
