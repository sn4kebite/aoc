package main

import (
	"bufio"
	"container/list"
	"fmt"
	"os"
	"sort"
)

const (
	ELF = 'E'
	GOBLIN = 'G'
)

type Entity struct {
	t rune
	hp int
	x, y int
}

type Tile struct {
	x, y int
	entity *Entity
	wall bool
}

type Step struct {
	tile *Tile
	parent *Step
}

func abs(a int) int {
	if a < 0 {
		return -a
	}
	return a
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	var m [][]*Tile
	var entities []*Entity
	line := 0
	for scanner.Scan() {
		var l []*Tile
		for x, c := range scanner.Text() {
			var e *Entity
			if c == ELF || c == GOBLIN {
				e = &Entity{c, 200, x, line}
				entities = append(entities, e)
			}
			l = append(l, &Tile{x, line, e, c == '#'})
		}
		m = append(m, l)
		line++
	}
	search_to := func(from, to *Tile) (steps []*Tile) {
		visited := make(map[int]bool)
		is_visited := func(x, y int) bool {
			return visited[y | (x << 16)]
		}
		visit := func(x, y int) {
			visited[y | (x << 16)] = true
		}
		queue := list.New()
		add_next := func(x, y int, parent *Step) bool {
			var new_steps []*Step
			if ! m[y-1][x].wall && (m[y-1][x].entity == nil || m[y-1][x] == to) && ! is_visited(x, y-1) {
				new_steps = append(new_steps, &Step{m[y-1][x], parent})
				visit(x, y-1)
			}
			if ! m[y+1][x].wall && (m[y+1][x].entity == nil || m[y+1][x] == to) && ! is_visited(x, y+1) {
				new_steps = append(new_steps, &Step{m[y+1][x], parent})
				visit(x, y+1)
			}
			if ! m[y][x-1].wall && (m[y][x-1].entity == nil || m[y][x-1] == to) && ! is_visited(x-1, y) {
				new_steps = append(new_steps, &Step{m[y][x-1], parent})
				visit(x-1, y)
			}
			if ! m[y][x+1].wall && (m[y][x+1].entity == nil || m[y][x+1] == to) && ! is_visited(x+1, y) {
				new_steps = append(new_steps, &Step{m[y][x+1], parent})
				visit(x+1, y)
			}
			sort.SliceStable(new_steps, func(i, j int) bool {
				//i_pos := new_steps[i].tile.y * len(m[0]) + new_steps[i].tile.x
				//j_pos := new_steps[j].tile.y * len(m[0]) + new_steps[j].tile.x
				i_dist := abs(new_steps[i].tile.x - to.x) + abs(new_steps[i].tile.y - to.y)
				j_dist := abs(new_steps[j].tile.x - to.x) + abs(new_steps[j].tile.y - to.y)
				return i_dist < j_dist
			})
			for _, s := range new_steps {
				//queue.PushFront(s)
				queue.PushBack(s)
			}
			return len(new_steps) > 0
		}
		add_next(from.x, from.y, nil)
		for queue.Len() > 0 {
			node := queue.Front()
			queue.Remove(node)
			step := node.Value.(*Step)
			if step.tile == to {
				for ; step != nil; step = step.parent {
					steps = append(steps, step.tile)
				}
				return
			}
			add_next(step.tile.x, step.tile.y, step)
		}
		return
	}
	search_next := func(entity *Entity) (target *Tile) {
		var targets []*Tile
		shortest_distance := 1000
		for _, e := range entities {
			if e.t != entity.t {
				targets = append(targets, m[e.y][e.x])
				distance := abs(entity.x - e.x) + abs(entity.y - e.y)
				shortest_distance = min(distance, shortest_distance)
			}
		}
		var routes [][]*Tile
		shortest_route := 1000
		for _, t := range targets {
			tile := m[t.y][t.x]
			distance := abs(tile.x - entity.x) + abs(tile.y - entity.y)
			if distance > shortest_distance {
				continue
			}
			steps := search_to(m[entity.y][entity.x], tile)
			if len(steps) > 0 {
				routes = append(routes, steps)
				if len(steps) < shortest_route {
					shortest_route = len(steps)
				}
			}
		}
		for _, r := range routes {
			if len(r) == shortest_route {
				target = r[len(r)-1]
				return
			}
		}
		return
	}
	actions := 1
	print_map := func() {
		for _, l := range m {
			for _, t := range l {
				if t.entity != nil {
					fmt.Printf("%c", t.entity.t)
				} else {
					if t.wall {
						fmt.Print("#")
					} else {
						fmt.Print(".")
					}
				}
			}
			fmt.Println()
		}
	}
	print_map()
	round := 0
	for ; actions > 0; round++ {
		actions = 0
		for _, entity := range entities {
			target := search_next(entity)
			tile := m[entity.y][entity.x]
			if target != nil {
				fmt.Println("target", target)
				if target.entity == nil {
					fmt.Println(entity, "moving to", target)
					tile.entity, target.entity = target.entity, tile.entity
					target.entity.x, target.entity.y = target.x, target.y
					target = search_next(entity)
				}
				if target.entity != nil {
					fmt.Println("attacking entity")
					target.entity.hp -= 3
					if target.entity.hp <= 0 {
						for i, e := range entities {
							if e == target.entity {
								entities = append(entities[:i], entities[i+1:]...)
								break
							}
						}
						target.entity = nil
					}
				}
				actions++
			}
		}
		/*for _, l := range m {
			for _, t := range l {
				if t.entity != nil {
					target := search_next(t.entity)
					if target != nil {
						fmt.Println("target", target)
						if target.entity != nil {
							fmt.Println("attacking entity")
							target.entity.hp -= 3
							if target.entity.hp <= 0 {
								for i, e := range entities {
									if e == target.entity {
										entities = append(entities[:i], entities[i+1:]...)
										break
									}
								}
								target.entity = nil
							}
						} else {
							fmt.Println(t.entity, "moving to", target)
							t.entity, target.entity = target.entity, t.entity
							target.entity.x, target.entity.y = target.x, target.y
						}
						actions++
					}
				}
			}
		}*/
		print_map()
	}
	fmt.Println("End at round", round)
	score := 0
	for _, e := range entities {
		score += e.hp
	}
	fmt.Println("Score:", score*round)
}
