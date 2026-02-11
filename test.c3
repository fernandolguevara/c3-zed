module grammar::coverage;

import std::io;
import std::collections::list;

/*
 * Grammar coverage executable for c3-zed.
 * Includes comments, contracts, declarations, expressions and statements.
 */

$assert true : "compile-time assertion";

faultdef BAD_STATE, EMPTY_INPUT;

alias IntPtr = int*;
typedef Score = int;

const int MAGIC = 7;
const String MSG = "grammar coverage";

struct Vec2 {
    int x;
    int y;
}

struct Player @packed {
    inline Vec2 pos;
    String name;
    int health;
}

struct Pair <Type> {
    Type left;
    Type right;
}

union Number {
    int i;
    float f;
}

bitstruct Flags : uint {
    bool alive : 0;
    bool visible : 1;
    uint team : 2..7;
}

const enum State : int {
    IDLE = 0,
    RUNNING = 1,
    DONE = 2,
}

cenum OpCode {
    ADD,
    SUB,
}

<*
 @param a : "Left operand."
 @param b : "Right operand."
 @require a >= 0 && b >= 0 : "Operands are non-negative in this test."
*>
fn int add(int a, int b) {
    return a + b;
}

fn int mul(int a, int b = 2) {
    return a * b;
}

fn bool Player.is_alive(Player* p) {
    return p.health > 0;
}

fn int square_short(int x) => x * x;

fn int fold_values(int[5] values) {
    int total = 0;
    for (int i = 0; i < 5; i++) {
        total += values[i];
    }
    return total;
}

fn int loop_demo(int limit) {
    int acc = 0;
    int i = 0;

    while (i < limit) {
        acc += i;
        i++;
    }

    do {
        acc += 1;
        limit--;
    } while (limit > 0);

    foreach (idx, value : (int[4]){ 1, 2, 3, 4 }) {
        if (idx == 2) {
            continue;
        }
        acc += value;
    }

    return acc;
}

<*
 @param value : "Switch input value."
 @require value >= 0 : "This demo uses only non-negative values."
*>
fn int switch_demo(int value) {
    int out = 0;
    switch (value) {
        case 0:
            out = 10;
            nextcase;
        case 1:
            out += 20;
            break;
        default:
            out = 99;
    }
    return out;
}

fn void defer_demo() {
    int x = 1;
    defer {
        x += 1;
    }
    x += 2;
}

fn int expr_demo() {
    int a = 3;
    int b = 4;
    int c = 0;

    c = (a + b) * square_short(2);
    c += (a > b) ? 1 : 2;
    c += (a < b) && (b > 0) ? 5 : 0;
    c += (int)3.5;

    int[5] arr = { 1, 2, 3, 4, 5 };
    c += arr[0];
    c += arr[1..3].len;

    Vec2 v = { .x = 10, .y = 20 };
    c += v.x + v.y;

    String s = `raw string`;
    if (s.len > 0) {
        c += 1;
    }

    return c;
}

fn int list_demo() {
    List{int} numbers;
    numbers.tinit();
    defer numbers.free();

    numbers.push(10);
    numbers.push(20);
    numbers.push_all((int[]){ 1, 2, 3 });

    int sum = 0;
    foreach (i, value : numbers.array_view()) {
        if (i == 0) {
            continue;
        }
        sum += value;
    }

    if (numbers.contains(10)) {
        sum += (int)numbers.len();
    }

    return sum;
}

fn int main() {
    Player p = { .pos = { .x = 5, .y = 6 }, .name = "zed", .health = 100 };
    Number n = { .i = 42 };
    Flags flags = { .alive = true, .visible = true, .team = 3 };
    State st = RUNNING;
    OpCode op = ADD;
    Pair{int} pair = { .left = 8, .right = 13 };

    int[5] values = { 1, 2, 3, 4, MAGIC };

    int result = 0;
    result += add(1, 2);
    result += mul(3);
    result += fold_values(values);
    result += loop_demo(3);
    result += switch_demo(1);
    result += expr_demo();
    result += list_demo();
    result += pair.left + pair.right;
    result += p.pos.x;
    result += n.i;
    result += flags.team;

    if (p.is_alive()) {
        result += 1;
    } else {
        return 1;
    }

    if (st == RUNNING && op == ADD) {
        io::printfn("%s => %d", MSG, result);
    }

    defer_demo();
    return 0;
}
