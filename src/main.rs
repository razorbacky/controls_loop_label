/*  */
fn main() {
    let mut count = 0;

    'counting_up : loop { // 바깥쪽 loop
        println!("count = {count}"); // #1 최초 1회 카운트를 시작해서 내부 loop로 진행한다.

        let mut remaining = 10;

        loop { // #2  remaining 을 10으로 시작한다.
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // #4 .3에서 루프를 돌아 9 가 된 remaining 은 여기서에서 break 한다.
            }
            if count == 2 {
                break 'counting_up; // count가 2가 되면 내부 루프를 해제하고, 외부 루프로 넘어간다.
            }
            remaining -= 1; // #3 내부 loop가 한 번 돌 때마다 remaining 에서 -1 을 진행한다.
        }
        count += 1;
    }
    println!("End count = {count}");
}
