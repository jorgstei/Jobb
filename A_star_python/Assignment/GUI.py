import pygame as pg;

from Astar import A_star_solver;
from Map import Map_Obj;

class GUI():
    def __init__(self, task, window_size):
        # Tuple containing window size in x and y
        self.task = task
        # Set up our map object
        self.map = Map_Obj(self.task)
        # Fill in start and goal pos
        self.map.set_cell_value(self.map.start_pos, 8, False)
        self.map.set_cell_value(self.map.goal_pos, 9, False)
        
        # Tuple containing the number of cells in the grid in x- and y-direction
        self.nOfCells = (len(self.map.get_maps()[0][0]), len(self.map.get_maps()[0]))

        #print("Map dimensions:", self.nOfCells)
        #print(self.map.get_maps()[0])
        #self.map.show_map(self.map.get_maps()[1])

        self.window_size = (self.closestNumber(window_size[0], self.nOfCells[0]), self.closestNumber(window_size[1], self.nOfCells[1]))

        # Pixel size of border around cells in the grid
        self.border_size = 0

        self.astar = A_star_solver(self.map)
        

    def run(self):
        # Pygame setup
        pg.init()
        surface = pg.display.set_mode((self.window_size[0], self.window_size[1]))
        pg.display.set_caption("A* Task " + str(self.task))
        clock = pg.time.Clock()
        # Pixel size of each cell
        horizontal_cell_size = self.window_size[0] // self.nOfCells[0]
        vertical_cell_size = self.window_size[1] // self.nOfCells[1]
        
        
        print("Len path:", len(self.astar.final_path))
        running = True
        run_speed = 20
        counter = 0
        while running:
            for event in pg.event.get():
                if event.type == pg.QUIT:
                    running = False
            
            # Draw black background
            background = pg.Rect(0, 0, self.window_size[0], self.window_size[1])
            pg.draw.rect(surface, (50, 50, 50), background)

            counter += run_speed
            # Delay such that we don't redraw immediately after drawing the final path
            if(counter > 20*run_speed + len(self.astar.history) + len(self.astar.final_path)):
                counter = 0
        
            for x in range(self.nOfCells[0]):
                for y in range(self.nOfCells[1]):  
                    # Draw rectangles spaced apart by self.border_size
                    rect = pg.Rect(x*horizontal_cell_size+self.border_size, y*vertical_cell_size+self.border_size, horizontal_cell_size-2*self.border_size, vertical_cell_size-2*self.border_size)
                    
                    rgb = (255,255,255)
                    cell_val = self.map.get_cell_value((y,x))
                   
                    # Set cell color based on what type of cell it is
                    if(cell_val == -1):
                        rgb = (211,33,45)
                    elif(cell_val == 1):
                        rgb = (215,215,215)
                    elif(cell_val == 2):
                        rgb = (166,166,166)
                    elif(cell_val == 3):
                        rgb = (96,96,96)
                    elif(cell_val == 4):
                        rgb = (36,36,36)
                    elif(cell_val == 8):
                        rgb = (255,0,255)
                    elif(cell_val == 9):
                        rgb = (0,128,255)

                    # Draw a new history node for each frame
                    index_of_matching_history_node = -1
                    try:
                        index_of_matching_history_node = self.astar.history.index((x,y))
                        # Exits to except here if it can't match x and y to a valid path node
                        if(index_of_matching_history_node != len(self.astar.history)-1 and index_of_matching_history_node <= counter):
                            rgb=(113,156,170)
                            
                    except ValueError:
                        pass

                    # Draw a new final path node for each new frame after the full history path is drawn
                    index_of_matching_path_node = -1
                    try:
                        index_of_matching_path_node = self.astar.final_path.index((x,y))
                        # Exits to except here if it can't match x and y to a valid path node
                        if(index_of_matching_path_node != len(self.astar.final_path)-1 and index_of_matching_path_node <= counter - len(self.astar.history)):
                            rgb=(0,255,0)
                            
                    except ValueError:
                        pass

                    

                    pg.draw.rect(surface, rgb, rect)

            pg.display.update()
            # Framerate
            clock.tick(60)
    
    # Source: https://www.geeksforgeeks.org/find-number-closest-n-divisible-m/
    # It's purpose is to round the window size to the closest number to the user input which is divisible by the amount of cells in the map
    def closestNumber(self, n, m) :
        # Find the quotient
        q = int(n / m)
        
        # 1st possible closest number
        n1 = m * q
        
        # 2nd possible closest number
        if((n * m) > 0) :
            n2 = (m * (q + 1))
        else :
            n2 = (m * (q - 1))
        
        # if true, then n1 is the required closest number
        if (abs(n - n1) < abs(n - n2)) :
            return n1
        
        # else n2 is the required closest number
        return n2

if __name__ == '__main__':
    
    validTask = False
    while not validTask:
        task = input("What task are you solving?\n")
        task = int(task)
        if(task > 0 and task < 6):
            validTask = True
        else:
            print("Only avaliable tasks are 1, 2, 3 and 4")

    gui = GUI(task, (800,800))
    gui.run()

    
    


    
    





