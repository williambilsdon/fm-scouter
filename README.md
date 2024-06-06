# fm-scouter

## Multithreaded Exploration

Using Criterion as the benchmarking tool I tested the current single threaded performance of the `parse_records` function. This function takes the list of records, weights and headers and outputs a list of `Player`. For the benchmarking I used the `ScoutedPlayers.csv` file which consists of 262 lines of real data from football manager.

#### Performance

*Single-threaded*: `[2.0888 ms 2.0971 ms 2.1057 ms]`
*Multi-threaded*: `[702.18 µs 704.00 µs 705.92 µs]`

The multi-threaded performance runs off a hard coded thread count of 4. The difference in performance is about 2.9x faster when parallelised.

Curiously the performance above was achieved with the code block below. Where the threads are joined in a seperate iterator to the one used to spawn the threads.

```rust
let thread_results: Vec<ScopedJoinHandle<'_, Result<Vec<Player>, ParserError>>> = chunks
    .map(move |chunk| s.spawn(|| parse_records(chunk, headers, weights)))
    .collect();

thread_results
    .into_iter()
    .map(|thread| thread.join().unwrap())
    .try_fold(Vec::new(), |mut acc, result| {
        acc.extend(result?);
        Ok(acc)
    }
```

In the below example the performance was actually `0.2ms` slower than the single-threaded solution.

```rust
let thread_results: Vec<ScopedJoinHandle<'_, Result<Vec<Player>, ParserError>>> = chunks
    .map(move |chunk| s.spawn(|| parse_records(chunk, headers, weights)))
    .map(|thread| thread.join().unwrap())
    .collect();

thread_results
    .into_iter()
    .try_fold(Vec::new(), |mut acc, result| {
        acc.extend(result?);
        Ok(acc)
    }
```

The reason for this is that iterators are lazily evaluated. They need to be consumed in order to evaluate. In this case `collect()` consumes the iterator and returns a Vec. By mapping and joining in the same iterator that we spawn threads we're joining before the threads have had a chance to startup. (I believe the exact behaviour needs clarifying).

If we `join()` after consuming the spawn iterator the threads will only be joined after they're evaluated and running.

... I think the above is vaguely correct. It's definitely because iterators are lazily evaluated but the *why* isn't fully understood by me yet.
