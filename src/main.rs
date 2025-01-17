// 문제가 해결되지 않을때에는 디버그 기능을 이용해라
// "루프 라벨로 여러 반복문 사이에 모호함 없애기"

fn main() {
    let mut count = 0;

    'counting_up : loop { // 바깥쪽 loop
        println!("count = {count}"); 

        let mut remaining = 10;

        loop { 
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; 
            }
            if count == 2 {
                break 'counting_up; // 바깥쪽의 counting_up 루프를 탈출하여 println!("End~~")로 간다."바깥쪽 루프 탈출에 주의"
            }
            remaining -= 1; 
        }
        count += 1;
    }
    println!("End count = {count}");
}
