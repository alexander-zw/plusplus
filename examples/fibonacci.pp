/**
 * This is some sample code written in the ++ programming language.
 */

* fibonacci_number(n) {
    $last = 0;
    $curr = 1;
    ($i = 0; i < n; i++)! {
        $$temp = curr;
        curr = last + curr;
        last = temp;
    }
    ~curr;
}

console.log(fibonacci_number(6));
