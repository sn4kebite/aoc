package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"time"
)

type Guard struct {
	minutes_asleep [60]int
	total_minutes_asleep int
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	guard_id := 0
	max_guard_id := 0
	max_time := 0
	var asleep_time time.Time
	guards := make(map[int]Guard)
	for scanner.Scan() {
		line := scanner.Text()
		re := regexp.MustCompile("^\\[([^\\]]+)\\] (.+)$")
		matches := re.FindAllStringSubmatch(line, -1)
		act_time, err := time.Parse("2006-01-02 15:04", matches[0][1])
		if err != nil {
			panic(err)
		}
		fmt.Println(act_time)
		fmt.Println(" ", matches[0][2])
		action := matches[0][2]
		if action == "falls asleep" {
			asleep_time = act_time
		} else if action == "wakes up" {
			asleep := act_time.Sub(asleep_time)
			fmt.Println("asleep for", asleep)
			guard := guards[guard_id]
			for m := asleep_time.Minute(); m < act_time.Minute(); m++ {
				guard.minutes_asleep[m]++
			}
			guard.total_minutes_asleep += int(asleep.Minutes())
			if guard.total_minutes_asleep > max_time {
				max_guard_id = guard_id
				max_time = guard.total_minutes_asleep
			}
			guards[guard_id] = guard
		} else {
			_, err := fmt.Sscanf(action, "Guard #%d begins shift", &guard_id)
			if err != nil {
				panic(err)
			}
		}
	}
	fmt.Printf("Guard %d has max time with %d minutes\n", max_guard_id, max_time)
	max_minute := 0
	max_minute_value := 0
	for i, v := range guards[max_guard_id].minutes_asleep {
		if v > max_minute_value {
			max_minute = i
			max_minute_value = v
		}
	}
	fmt.Printf("Max minute: %d  Value: %d  Strategy 1 value: %d\n", max_minute, max_minute_value, max_guard_id * max_minute)
	max_minute = 0
	max_minute_value = 0
	max_guard_id = 0
	for guard_id, guard := range guards {
		for m, v := range guard.minutes_asleep {
			if v > max_minute_value {
				max_minute = m
				max_minute_value = v
				max_guard_id = guard_id
			}
		}
	}
	fmt.Printf("Max minute: %d  Value: %d  Strategy 2 value: %d\n", max_minute, max_minute_value, max_guard_id * max_minute)
}
