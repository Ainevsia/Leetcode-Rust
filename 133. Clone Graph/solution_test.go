package main 

import (
    "testing"
)

func Test1(t *testing.T) {
    node1 := new(Node)
    node1.Val = 1
    node2 := new(Node)
    node2.Val = 2
    node1.Neighbors = append(node1.Neighbors,node2)
    node2.Neighbors = append(node2.Neighbors,node1)
    res := cloneGraph(node1)
    if res == node1 {
        t.Fatalf(``)
    }
}

