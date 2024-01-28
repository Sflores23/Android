# Example usage with a specific prime p, and coefficients a, b
p = 433
a = 6
b = 15

def calculate_points(p, a, b):
    points = []
    for x in range(p):
        y_squared = (x**3 + a * x + b) % p
        for y in range(p):
            if (y * y) % p == y_squared:
                point = (x, y)
                if point not in points:
                      points.append(point)
    return points


def point_addition(p, a, b, x1, y1, x2, y2):
    # Step 1: Calculate the slope of the line passing through P and Q
    if x1 == x2 and y1 == y2 and y1 != 0:  # Point doubling
        m = ((3 * x1 * x1 + a) * pow(2 * y1, -1, p)) % p
    elif x1 != x2:  # Point addition
        m = ((y2 - y1) * pow(x2 - x1, -1, p)) % p
    else:  # Points are not distinct, return infinity point
        return None, None

    # Step 2: Calculate the intersection point R = P + Q
    x3 = (m * m - x1 - x2) % p
    y3 = (m * (x1 - x3) - y1) % p

    return x3, y3


def all_point_sums(p, a, b, points):
    sums = []
    for i in range(len(points)):
        for j in range(i,len(points)):  # Start from i to avoid redundant calculations
            sum_point = point_addition(p, a, b, points[i][0], points[i][1], points[j][0], points[j][1])
            if sum_point in points or sum_point == (None, None):
                print(f"{points[i]} + {points[j]} = {sum_point}")
                sums.append((points[i], points[j], sum_point))
    return sums


def point_order(p, a, b, x, y):
    current_x, current_y = x, y
    order = 2  # Initialize the order as 2 bc 1*(x1,y1) is itself

    while True:
        # Calculate the next point in the sequence
        next_x, next_y = point_addition(p, a, b, x, y, current_x, current_y)
        if next_x is None and next_y is None:  # If it's the point at infinity
            return order  # Return the order
        current_x, current_y = next_x, next_y
        order += 1
        
        



counter = 1
all_points = calculate_points(p, a, b)
print("\n\nAll points on the elliptic curve:\n")

print(f"Point {counter}:(None, None)")
for point in all_points:
    counter += 1
    print(f"Point {counter}: {point}")

print()
print()

for point in all_points:
    x, y = point
    order = point_order(p, a, b, x, y)
    print(f"Order of point {point}: {order}")
    
print()
print()

all_sums = all_point_sums(p, a, b, all_points)



import matplotlib.pyplot as plt
def plot_elliptic_curve(p, a, b):
    points = calculate_points(p, a, b)
    x_coords, y_coords = zip(*points)

    plt.scatter(x_coords, y_coords, color='blue')

    for i, point in enumerate(points):
        plt.text(point[0], point[1], f'({point[0]},{point[1]})', ha='left')

    plt.title(f'Elliptic Curve y^2 = x^3 + {a}x + {b} mod {p}')
    plt.xlabel('x')
    plt.ylabel('y')
    plt.legend()
    plt.grid(True)
    plt.show()

plot_elliptic_curve(p, a, b)
