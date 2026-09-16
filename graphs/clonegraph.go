package main

type Node struct {
	Val       int
	Neighbors []*Node
}

func cloneGraph(node *Node) *Node {
	if node == nil {
		return nil
	}
	visited := make(map[*Node]*Node)

	var dfs func(neighbor *Node) *Node

	dfs = func(pNode *Node) *Node {
		if cloned, exists := visited[pNode]; exists {
			return cloned
		}
		clone := &Node{Val: pNode.Val}

		visited[pNode] = clone

		for _, neighbor := range pNode.Neighbors {
			clone.Neighbors = append(clone.Neighbors, dfs(neighbor))
		}
		return clone
	}
	return dfs(node)
}
