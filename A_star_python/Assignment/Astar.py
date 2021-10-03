import math
#f(n) = g(n) + h(n) where g(n) is the path cost from intial state to node n, and h(n) is the estimated cost to the goal

def heuristic_func(curr_pos, goal_pos):
    # Ty Pythagoras
    return math.sqrt(abs(curr_pos[0] - goal_pos[0])**2 + abs(curr_pos[1] - goal_pos[1])**2)

class Node():
    def __init__(self, pos, cell_type, g_cost=99999):
        self.pos = pos
        self.edges = []
        self.prev_node = None
        self.g_cost = g_cost
        # Cost corresponds to number. Acceptable inputs are 1,2,3,4,8,9
        self.cell_type = cell_type
    
    def add_edge(self, edge_to_add, weight):
        #print("Adding edge")
        self.edges.append(Edge(edge_to_add, weight))

    # pos is a tuple of x,y coordinates
    # returns a tuple with a boolean (for whether the node is a neighbour or not) and weight of the edge between the nodes
    def node_is_neighbour(self, node):
        # Node is below or above self
        if(self.pos[0] == node.pos[0] and (self.pos[1] + 1 == node.pos[1] or self.pos[1] - 1 == node.pos[1])):
            return (True, node.cell_type)
        # Node is left or right of self
        elif(self.pos[1] == node.pos[1] and (self.pos[0] + 1 == node.pos[0] or self.pos[0] - 1 == node.pos[0])):
            return (True, node.cell_type)

        return (False, -1)

    def to_string(self):
        return "Node at pos (%d, %d) with g-cost %d" % (self.pos[0], self.pos[1], self.g_cost)

    def print_all_edges(self):
        for edge in self.edges:
            print(self.to_string(), " || ", edge.to_string())

class Edge():
    def __init__(self, target_node, weight):
        self.target = target_node
        self.weight = weight
    def to_string(self):
        return "Edge: ", self.target.to_string(), "with weight", self.weight

class PriorityQueue():
    def __init__(self, eval_func, end_pos):
        self.goal_pos = end_pos
        self.queue = []
        self.eval = eval_func

    # Add element to queue in it's correct place
    def add_element(self, element_to_add):
        # Insert element such that the queue stays in prioritized order
        if(len(self.queue) > 0):
            for i in range(len(self.queue)):
                node = self.queue[i]
                #print(node.g_cost + self.eval(node.pos, self.goal_pos), "vs" ,element_to_add.g_cost + self.eval(element_to_add.pos, self.goal_pos))
                if(node.g_cost + self.eval(node.pos, self.goal_pos) > element_to_add.g_cost + self.eval(element_to_add.pos, self.goal_pos)):
                    self.queue.insert(i, element_to_add)
                    return True
            
            self.queue.append(element_to_add)
            return True
            
        else:
            #print("Appended in else")
            self.queue.append(element_to_add)
            return True
    # Remove element from queue
    def remove_element(self, element_to_remove):
        # Find element and remove it
        if(len(self.queue) == 0):
            print("Cannot remove element since queue is empty")
            return False
        else:
            for i in range(len(self.queue)):
                node = self.queue[i]
                if(node.pos == element_to_remove.pos):
                    self.queue.pop(i)
                    return True
        return False
    # Check if node already is in the queue
    def element_exists_in_queue(self, element):
        for node in self.queue:
            if(node.pos == element.pos):
                return True
        else:
            return False
    # Update priority queue such that it is in the correct order
    def reorder_queue(self):
        self.queue.sort(key=self.sortBy)

    # Helper function for the inbuilt python sort
    def sortBy(self, obj):
        return obj.g_cost + self.eval(obj.pos, self.goal_pos)

    def to_string(self):
        output = "Queue:\n"
        for node in self.queue:
            output += "Node at pos: (%d, %d) with %d edges.\n" % (node.pos[0],node.pos[1], len(node.edges))
        return output

class A_star_solver():
    # -1 in map is non-traversable tile, 1 is traversable tile, 2 is start pos, 3 is end pos
    def __init__(self, map):
        
        self.map_obj = map
        self.map = self.map_obj.get_maps()[0]
        #map.print_map(map.get_maps()[0])
        # Array that holds the final solution of the A*
        self.final_path = []
        # Complete history of every node that has been opened by the algorithm
        self.history = []

        self.nodes = []
        # Create every valid node in map and store it in self.nodes
        for i in range(len(self.map[0])):
            for j in range(len(self.map)):
                if(self.map[j][i] != -1):
                    node = Node((i,j), self.map[j][i])
                    self.nodes.append(node)
                    # Start node
                    if (self.map[j][i] == 8):
                        self.start_node = node
                    # End node
                    elif (self.map[j][i] == 9):
                        self.end_node = node

        self.prio_queue = PriorityQueue(heuristic_func, self.end_node.pos)
        '''
        for node in self.nodes:
            self.map_obj.set_cell_value((node.pos[1], node.pos[0]), 4, False)
        self.map_obj.print_map(self.map_obj.get_maps()[0])
        '''
            
        

        # Fill in edges for each node
        for node in self.nodes:
            for potential_neighbour in self.nodes:
                is_neighbour, weight = node.node_is_neighbour(potential_neighbour)
                if(is_neighbour):
                    node.add_edge(potential_neighbour, weight)

        # Add start node to priority queue and set it's g_cost to 0
        self.prio_queue.add_element(self.start_node)
        self.start_node.g_cost = 0
        
        
        #'''
        # List to hold all nodes we have already opened (except those that update their g_cost by finding a better route)
        closed_list = []
        while(len(self.prio_queue.queue) != 0):
            # Open the first node in the queue
            current = self.prio_queue.queue[0]
            # If we reached end, reconstruct path and break
            if(current.pos == self.end_node.pos):
                print("Found end", current.to_string())
                self.final_path = self.reconstruct_path(current)
                self.history.pop(0)
                print("Path:\n", self.final_path)
                break
            # Add node to history and closed_list, and remove it from the prio queue
            self.history.append(current.pos)
            self.prio_queue.remove_element(current)
            closed_list.append(current)
            
            #print("Current node is: ", current.to_string())

            for edge in current.edges:
                    g_cost_of_edge = current.g_cost + edge.weight
                    # Check if this path is the most efficient way to get to the edge target so far
                    if(g_cost_of_edge < edge.target.g_cost):
                        edge.target.prev_node = current
                        edge.target.g_cost = g_cost_of_edge
                        self.prio_queue.remove_element(edge.target)
                        try:
                            closed_list.remove(edge.target)
                        except ValueError:
                            pass
                        


                    #print("Node to add", edge.target.to_string())
                    # Check if edge target is already opened
                    in_closed = -1
                    try:
                        in_closed = closed_list.index(edge.target)
                    except ValueError:
                        pass
                    # If it's not already opened, add it to the queue
                    if(not self.prio_queue.element_exists_in_queue(edge.target) and in_closed == -1):
                        #print("Before add", self.prio_queue.to_string())
                        self.prio_queue.add_element(edge.target)
                        #print("After add:", self.prio_queue.to_string())
                        
                    
                    else:
                        self.prio_queue.reorder_queue()
                        
        #'''
    def reconstruct_path(self, goal_node):
        path = []
        current = goal_node
        # Loop through each node from the goal node and add their prev_node to the array
        while(current.prev_node != None):
            path.insert(0, current.pos)
            current = current.prev_node
        return path
        

    

        

        





