package main

import (
	"bufio"
	"fmt"
	"os"
)

const (
	horizontal	= '-'
	vertical	= '|'
	intersect	= '+'
	corner_f	= '/'
	corner_b	= '\\'
)

const (
	left	= '<'
	right	= '>'
	up		= '^'
	down	= 'v'
	none	= ' '
)

const (
	turn_left = iota
	turn_straight = iota
	turn_right = iota
	max_turns = iota
)

type Tile struct {
	tile rune
	player rune
	tick int
	turn int
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	var carts [][]*Tile
	for scanner.Scan() {
		var line []*Tile
		for _, c := range scanner.Text() {
			p := ' '
			if c == left || c == right || c == up || c == down {
				p = c
				if p == left || p == right {
					c = horizontal
				} else {
					c = vertical
				}
			}
			line = append(line, &Tile{c, p, 0, turn_left})
		}
		carts = append(carts, line)
	}
	print := func() {
		for _, line := range carts {
			for _, t := range line {
				if t.player != none {
					fmt.Printf("%c", t.player)
				} else {
					fmt.Printf("%c", t.tile)
				}
			}
			fmt.Println()
		}
	}
	tick := 1
	rotate_left := func(c rune) rune {
		switch c {
		case left: return down
		case right: return up
		case up: return left
		case down: return right
		}
		return c
	}
	rotate_right := func(c rune) rune {
		switch c {
		case left: return up
		case right: return down
		case up: return right
		case down: return left
		}
		return c
	}
	move := func(fx, fy int, remove bool) bool {
		t := carts[fy][fx]
		direction := t.player
		tx, ty := fx, fy
		if direction == left {
			tx--
		} else if direction == right {
			tx++
		} else if direction == up {
			ty--
		} else if direction == down {
			ty++
		}
		tt := carts[ty][tx]
		if tt.player != ' ' {
			if remove {
				t.player = ' '
				tt.player = ' '
				return true
			} else {
				panic(fmt.Sprintf("boom at %d,%d", tx, ty))
			}
		}
		tt.turn = t.turn
		if tt.tile == corner_f {
			if t.player == left {
				tt.player = down
			} else if t.player == right {
				tt.player = up
			} else if t.player == up {
				tt.player = right
			} else if t.player == down {
				tt.player = left
			}
		} else if tt.tile == corner_b {
			if t.player == left {
				tt.player = up
			} else if t.player == right {
				tt.player = down
			} else if t.player == up {
				tt.player = left
			} else if t.player == down {
				tt.player = right
			}
		} else if tt.tile == '+' {
			switch t.turn {
			case turn_left:
				tt.player = rotate_left(t.player)
			case turn_right:
				tt.player = rotate_right(t.player)
			default:
				tt.player = t.player
			}
			tt.turn = (t.turn + 1) % 3
		} else {
			tt.player = t.player
		}
		tt.tick = tick
		t.player = ' '
		return false
	}
	print()
	for {
		num_carts := 0
		for y, line := range carts {
			for x, t := range line {
				if t.player != none && t.tick < tick {
					move(x, y, true)
				}
			}
		}
		tick++
		cx, cy := 0, 0
		for y, line := range carts {
			for x, t := range line {
				if t.player != ' ' {
					num_carts++
					cx, cy = x, y
				}
			}
		}
		if num_carts == 1 {
			print()
			fmt.Printf("%d,%d\n", cx, cy)
			break
		}
	}
}
