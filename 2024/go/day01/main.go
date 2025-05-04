package main

import (
    "os"
    "bufio"
    "io"
    "fmt"
    "log"
    "slices"
    "strings"
    "strconv"
)

func parse(input string) [2][]int {
    scanner := bufio.NewScanner(strings.NewReader(input))

    a := make([]int, 0)
    b := make([]int, 0)

    for scanner.Scan() {
        ab := strings.Split(scanner.Text(), "   ")
        next_a, _ := strconv.Atoi(ab[0])
        next_b, _ := strconv.Atoi(ab[1])
        a = append(a, next_a)
        b = append(b, next_b)
    }

    return [2][]int{a, b}
}

func solve_a(input string) int {
    lists := parse(input)
    slices.Sort(lists[0])
    slices.Sort(lists[1])

    total := 0

    for i, v := range lists[0] {
        total += max(v - lists[1][i], lists[1][i] - v)
    }

    return total
}

func solve_b(input string) int {
    lists := parse(input)
    m := make(map[int]int)
    for _, k := range lists[1] {
        v, _ := m[k]
        m[k] = v + 1
    }

    total := 0
    for _, k := range lists[0] {
        v, _ := m[k]
        total += k * v
    }
    return total
}


func main() {
    reader := bufio.NewReader(os.Stdin)
    b, err := io.ReadAll(reader)
    if err != nil {
        log.Fatal(err)
    }
    input := string(b[:])

    fmt.Println("Solution A-part", solve_a(input))
    fmt.Println("Solution B-part", solve_b(input))
}
