package main

import (
    "os"
    "bufio"
    "fmt"
    "strconv"
    "strings"
)

type command struct {
    direction string
    dist int
}

type coord struct {
    x int
    y int
}

func get_input() []command {
    commands := make([]command, 0)

    scanner := bufio.NewScanner(os.Stdin)

    for {
        scanner.Scan()
        raw_command := scanner.Text()

        if len(raw_command) == 0 { break }

        split_command := strings.Split(raw_command, " ")
        dist, _ := strconv.Atoi(split_command[1])

        commands = append(commands, command{
            direction: split_command[0],
            dist: dist,
        })
    }

    return commands
}

func solve_a(commands []command) int {
    pos := coord{ x: 0, y: 0}

    for _, command := range commands {
        switch command.direction {
            case "forward":
                pos.x += command.dist
            case "down":
                pos.y += command.dist
            case "up":
                pos.y -= command.dist
        }
    }

    return pos.x * pos.y
}

func solve_b(commands []command) int {
    pos := coord{ x: 0, y: 0}
    aim := 0

    for _, command := range commands {
        switch command.direction {
            case "forward":
                pos.x += command.dist
                pos.y += command.dist * aim
            case "down":
                aim += command.dist
            case "up":
                aim -= command.dist
        }
    }

    return pos.x * pos.y
}


func main() {
    numbers := get_input()

    fmt.Println("Solution A-part", solve_a(numbers))
    fmt.Println("Solution B-part", solve_b(numbers))
}
