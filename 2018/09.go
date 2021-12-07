package main

import (
	"container/ring"
	"fmt"
	"io/ioutil"
	"os"
)

type Marble struct {
	value int
}

type Player struct {
	number int
	points int
}

func main() {
	buffer, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}
	var player_count, last_points int
	fmt.Sscanf(string(buffer), "%d players; last marble is worth %d points", &player_count, &last_points)
	fmt.Printf("Marble game with %d players, %d marbles\n", player_count, last_points)
	players := ring.New(player_count)
	for i := 0; i < player_count; i++ {
		players.Value = &Player{i, 0}
		players = players.Next()
	}
	circle := ring.New(1)
	circle.Value = &Marble{0}
	for marble_counter := 1; marble_counter <= last_points; marble_counter++ {
		current_player := players.Value.(*Player)
		players = players.Next()
		if marble_counter % 23 == 0 {
			current_player.points += marble_counter
			circle = circle.Move(-7)
			current_player.points += circle.Value.(*Marble).value
			circle = circle.Move(-1)
			circle.Unlink(1)
			circle = circle.Move(1)
			continue
		}
		circle = circle.Next()
		new_marble := ring.New(1)
		new_marble.Value = &Marble{marble_counter}
		circle.Link(new_marble)
		circle = circle.Next()
		/*fmt.Printf("[%d] ", current_player.number+1)
		circle.Do(func(e interface{}) {
			m := e.(*Marble)
			circle = circle.Next()
			if m == new_marble.Value.(*Marble) {
				fmt.Printf("(%d) ", m.value)
			} else {
				fmt.Print(m.value, " ")
			}
		})
		fmt.Println()*/
	}
	var high *Player
	for i := 0; i < player_count; i++ {
		player := players.Value.(*Player)
		players = players.Next()
		if high == nil || player.points > high.points {
			high = player
		}
	}
	fmt.Printf("High score: Player %d with %d points\n", high.number, high.points)
}
