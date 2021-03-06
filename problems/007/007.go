package main

import "bufio"
import "fmt"
import "os"
import "strconv"
import "time"

func solution() uint32 {
    i, map_, n := uint32(0), make(map[uint32]uint32), uint32(10000)

    for q := uint32(2);; q++ {
        p := map_[q]

        if p == 0 {
            map_[q * q] = q

            if i == n {
                return q
            } else {
                i += 1
            }
        } else {
            x := p + q

            for map_[x] != 0 {
                x += p
            }

            map_[x] = p
        }
    }
}

func main() {
    args := os.Args

    if len(args) == 2 && args[1] == "-a" {
        fmt.Println(solution())
        return
    }

    scanner := bufio.NewScanner(os.Stdin)
    for scanner.Scan() {
        iters, _ := strconv.Atoi(scanner.Text())

        start := time.Now()
        for i := 0; i < iters; i++ {
            solution()
        }
        end := time.Now()

        fmt.Println(end.Sub(start).Nanoseconds())
    }
}
