// ============================================================
// Rust 1단계: 소유권(Ownership) & 빌림(Borrowing) & impl
// ============================================================

fn main() {
    ownership_basics();
    move_semantics();
    borrowing();
    mutable_borrow();
    clone_vs_copy();
    block_scoped_borrow();
    deep_vs_shallow_clone();
    impl_basics();
}

// ----------------------------------------------------------
// 1. 소유권 기초
// 힙 데이터(String 등)는 딱 하나의 소유자만 존재
// 소유자가 scope를 벗어나면 자동으로 메모리 해제 (drop)
// → C++ RAII와 동일한 개념, 단 컴파일러가 강제
// ----------------------------------------------------------
fn ownership_basics() {
    println!("\n--- 1. 소유권 기초 ---");

    let s = String::from("hello"); // s가 "hello"의 소유자
    println!("s = {s}");

    // s가 scope를 벗어나면 자동으로 drop() 호출 → free()
    // C++의 소멸자와 동일, 단 명시적 delete 불필요
}

// ----------------------------------------------------------
// 2. Move (소유권 이전)
// Python처럼 변수 대입해도 복사가 아님 (힙 데이터 기준)
// C++ std::move와 같은 효과가 대입 시 자동 발생
// → 이전 변수는 더 이상 사용 불가 (컴파일 에러)
// ----------------------------------------------------------
fn move_semantics() {
    println!("\n--- 2. Move ---");

    let s1 = String::from("world");
    let s2 = s1; // s1 → s2 로 소유권 이동 (move)

    // println!("{s1}"); // 컴파일 에러! s1은 이미 무효
    println!("s2 = {s2}"); // s2만 유효

    // 함수 인자로 넘기면 소유권이 함수 내부로 이동
    takes_ownership(s2);
    // println!("{s2}"); // 컴파일 에러! s2도 이제 무효
}

fn takes_ownership(s: String) {
    println!("함수 내부에서: {s}");
    // 함수 끝 → s drop
}

// ----------------------------------------------------------
// 3. 불변 빌림 (&)
// 소유권을 넘기지 않고 참조만 빌려줌
// → C++의 const 참조(const T&)와 동일
// 동시에 여러 개의 불변 참조 허용
// ----------------------------------------------------------
fn borrowing() {
    println!("\n--- 3. 불변 빌림 (&) ---");

    let s = String::from("rust");

    let len = calculate_length(&s); // 소유권 유지, 참조만 전달
    println!("'{s}'의 길이: {len}"); // s 여전히 유효
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // s는 참조라 drop 안 됨, 소유권도 없음
}

// ----------------------------------------------------------
// 4. 가변 빌림 (&mut)
// 수정 가능한 참조
// → C++의 T&와 동일, 단 규칙이 엄격:
//   "가변 참조는 동시에 딱 하나만 존재 가능"
//   (data race를 컴파일 타임에 원천 차단)
// ----------------------------------------------------------
fn mutable_borrow() {
    println!("\n--- 4. 가변 빌림 (&mut) ---");

    let mut s = String::from("hello");
    append_world(&mut s);
    println!("수정 후: {s}");

    // 동시에 두 개의 &mut는 불가:
    // let r1 = &mut s;
    // let r2 = &mut s; // 컴파일 에러!
}

fn append_world(s: &mut String) {
    s.push_str(", world");
}

// ----------------------------------------------------------
// 5. Clone vs Copy
// Clone: 힙 데이터 명시적 깊은 복사 (비용 있음)
// Copy:  스택 데이터(i32, f64, bool 등)는 자동 복사
//        → Python 기본 자료형과 동일한 동작
// ----------------------------------------------------------
fn clone_vs_copy() {
    println!("\n--- 5. Clone vs Copy ---");

    // Clone: 명시적 복사 (String은 힙 데이터)
    let s1 = String::from("clone me");
    let s2 = s1.clone(); // 깊은 복사
    println!("s1={s1}, s2={s2}"); // 둘 다 유효

    // Copy: 스택 데이터는 자동 복사
    let x = 42;
    let y = x; // move 아님, 그냥 복사
    println!("x={x}, y={y}"); // 둘 다 유효
}

// ----------------------------------------------------------
// 6. 블록 스코프로 &mut 수명 제어
// {} 블록이 끝나면 참조가 소멸 → 새 &mut 허용
// NLL(Non-Lexical Lifetimes): 블록 없이도 마지막 사용
// 시점 기준으로 컴파일러가 자동 판단
// ----------------------------------------------------------
fn block_scoped_borrow() {
    println!("\n--- 6. 블록 스코프 &mut ---");

    let mut s = String::from("hello");

    // 방법 1: 블록으로 명시적 수명 종료
    {
        let r1 = &mut s;
        r1.push_str("!");
    } // r1 소멸 → &mut 잠금 해제

    let r2 = &mut s; // OK
    r2.push_str("?");
    println!("블록 방식: {s}"); // hello!?

    // 방법 2: NLL - 블록 없이도 마지막 사용 후 수명 종료
    let mut s2 = String::from("hello");

    let r3 = &mut s2;
    r3.push_str("!"); // r3 마지막 사용 → 수명 자동 종료

    let r4 = &mut s2; // OK (r3 이미 끝났으므로)
    r4.push_str("?");
    println!("NLL 방식:  {s2}"); // hello!?
}

// ----------------------------------------------------------
// 7. 깊은 복사 vs 얕은 복사 (주소로 확인)
// derive(Clone): 모든 필드 재귀적으로 새로 할당
// 수동 Clone + raw pointer: 주소만 복사 → 같은 힙 가리킴
//   (unsafe 영역, 실제로는 거의 안 씀)
// ----------------------------------------------------------
#[derive(Clone)]
struct Deep {
    name: String,
    scores: Vec<i32>,
}

struct Shallow {
    name: String,
    scores_ptr: *const Vec<i32>, // raw pointer: 주소만 저장
}

impl Shallow {
    fn new(name: &str, scores: &Vec<i32>) -> Self {
        Shallow {
            name: String::from(name),
            scores_ptr: scores as *const Vec<i32>,
        }
    }
}

impl Clone for Shallow {
    fn clone(&self) -> Self {
        Shallow {
            name: self.name.clone(),     // name은 새로 복사
            scores_ptr: self.scores_ptr, // 주소만 복사 → 같은 힙!
        }
    }
}

fn deep_vs_shallow_clone() {
    println!("\n--- 7. 깊은 복사 vs 얕은 복사 ---");

    // derive(Clone): 완전한 깊은 복사
    let a = Deep { name: String::from("Alice"), scores: vec![10, 20, 30] };
    let b = a.clone();

    println!("[깊은 복사 - derive(Clone)]");
    println!("  a.scores 주소: {:p}", a.scores.as_ptr());
    println!("  b.scores 주소: {:p}", b.scores.as_ptr());
    println!("  같은 주소?    {}", a.scores.as_ptr() == b.scores.as_ptr()); // false

    // 수동 Clone + raw pointer: 얕은 복사
    let scores = vec![10, 20, 30];
    let c = Shallow::new("Bob", &scores);
    let d = c.clone();

    println!("[얕은 복사 - 수동 Clone + raw pointer]");
    println!("  c.scores_ptr: {:p}", c.scores_ptr);
    println!("  d.scores_ptr: {:p}", d.scores_ptr);
    println!("  같은 주소?   {}", c.scores_ptr == d.scores_ptr); // true → 위험!
}

// ----------------------------------------------------------
// 8. impl 기초
// struct: 데이터 정의
// impl:   해당 struct의 메서드 정의 (동작을 밖에서 붙임)
//
// C++ → class 안에 데이터+메서드 함께
// Python → class 안에 def로 함께
// Rust → struct(데이터) + impl(동작) 분리
// ----------------------------------------------------------

struct Player {
    name: String,
    hp: i32,
    attack: i32,
}

impl Player {
    // 연관 함수 (associated function): &self 없음
    // → C++ static 멤버함수 / Python @staticmethod
    // → 보통 생성자로 사용, Self = Player 타입 자기 자신
    fn new(name: &str, hp: i32, attack: i32) -> Self {
        Player {
            name: String::from(name),
            hp,        // 변수명과 필드명이 같으면 축약 가능 (Python: 없는 문법)
            attack,
        }
    }

    // 불변 메서드: &self → 읽기만 가능
    // → C++ const 멤버함수 / Python self
    fn status(&self) {
        println!("  [{}] HP:{} ATK:{}", self.name, self.hp, self.attack);
    }

    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    // 가변 메서드: &mut self → 필드 수정 가능
    // → C++ 일반 멤버함수 / Python self (수정)
    fn take_damage(&mut self, dmg: i32) {
        self.hp -= dmg;
        if self.hp < 0 { self.hp = 0; }
    }

    fn attack_target(&self, target: &mut Player) {
        println!("  {} → {} 공격! ({}dmg)", self.name, target.name, self.attack);
        target.take_damage(self.attack);
    }
}

fn impl_basics() {
    println!("\n--- 8. impl 기초 ---");

    // new()로 생성 (파이썬 __init__ / C++ 생성자)
    let mut hero   = Player::new("영웅", 100, 25);
    let mut goblin = Player::new("고블린", 40, 10);

    println!("[초기 상태]");
    hero.status();
    goblin.status();

    // 메서드 호출
    hero.attack_target(&mut goblin);
    goblin.attack_target(&mut hero);

    println!("[전투 후]");
    hero.status();
    goblin.status();

    println!("영웅 생존? {}", hero.is_alive());
    println!("고블린 생존? {}", goblin.is_alive());
}
