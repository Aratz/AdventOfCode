package main

import (
    "os"
    "bufio"
    "io"
    "fmt"
    "log"
)

func solve_a(input string) int {
    return 0
}

func solve_b(input string) int {
    return 0
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
