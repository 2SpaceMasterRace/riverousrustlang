# Tests

Generally, we try to keep our tests next to the code that it's testing in the Rust community. However we usually make a new namespace for the tests, to avoid conflicting with the "real" code. Just as we used mod to specify that first.rs should be included in lib.rs, we can use mod to basically create a whole new file inline.

We'll do that with the assert_eq! macro. This isn't some special testing magic. All it does is compare the two things you give it, and panic the program if they don't match. Yep, you indicate failure to the test harness by freaking out!

To appease the compiler (and to be friendly to our consumers), we should indicate that the whole test module should only be compiled if we're running tests.
