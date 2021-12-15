package main

import (
    "os"
    "bufio"
    "fmt"
    "strconv"
)

func get_input() []int {
    numbers := make([]int, 0)

    scanner := bufio.NewScanner(os.Stdin)

    for {
        scanner.Scan()
        s_n := scanner.Text()

        if len(s_n) == 0 { break }

        n, _ := strconv.Atoi(s_n)
        numbers = append(numbers, n)
    }

    return numbers
}

func solve_a(numbers []int) int {
    count := 0

    for i, n:= range numbers[:len(numbers) - 1] {
        if n < numbers[i + 1] {
            count += 1
        }
    }

    return count
}

func solve_b(numbers []int) int {
    avg_numbers := make([]int, len(numbers) - 2)
    for i, _:= range avg_numbers {
        avg_numbers[i] = numbers[i] + numbers[i + 1] + numbers[i + 2]
    }

    return solve_a(avg_numbers)
}


func main() {
    numbers := get_input()

    fmt.Println("Solution A-part", solve_a(numbers))
    fmt.Println("Solution B-part", solve_b(numbers))
}
