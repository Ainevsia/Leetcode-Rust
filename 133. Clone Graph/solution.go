package main

import (
    "fmt"
)

type Node struct {
    Val int
    Neighbors []*Node
}

func cloneGraph(node *Node) *Node {
    if node == nil { return node }  // deal with nil case
    m := make(map[int]*Node)
    visited := make(map[int]bool)
    // init queue
    q := make([]*Node,1)
    q[0] = node
    // bfs
    for len(q) != 0 {
        curNode := q[0]
        q = q[1:]
        // append to m if curNode not visited yet
        if m[curNode.Val] == nil {
            m[curNode.Val] = new(Node)
            m[curNode.Val].Val = curNode.Val
        }
        for _, j := range curNode.Neighbors {
            if m[j.Val] == nil {
                // not visited yet, append to q
                q = append(q,j)
            }
        }
    }
    q = append(q,node)
    visited[node.Val] = true
    for len(q) != 0 {
        // bfs
        oldNode := q[0]
        q = q[1:]
        newNode := m[oldNode.Val]
        for _, j := range oldNode.Neighbors {
            nbidx := j.Val
            newNode.Neighbors = append(newNode.Neighbors, m[nbidx])

            // push q
            if visited[j.Val] == false {
                q = append(q,j)
                visited[j.Val] = true
            }
        }
    }
    return m[node.Val]
}

func main() {
    node1 := new(Node)
    node1.Val = 1
    node2 := new(Node)
    node2.Val = 2
    node1.Neighbors = append(node1.Neighbors,node2)
    node2.Neighbors = append(node2.Neighbors,node1)
    res := cloneGraph(node1)
    fmt.Println(res)
    fmt.Println(node1)
    fmt.Println(node2)
}
