import shapely.geometry
import matplotlib.pyplot as plt

def tobox(x,y,z):
    print(f"{x} {y} {z}")

    p000 = (x[0],y[0],z[0])
    p001 = (x[0],y[0],z[1])
    p010 = (x[0],y[1],z[0])
    p011 = (x[0],y[1],z[1])
    p000 = (x[0], y[0], z[0])
    p101 = (x[1], y[0], z[1])
    p110 = (x[1], y[1], z[0])
    p111 = (x[1], y[1], z[1])

    box = shapely.geometry.Polygon([
        p000,
        p001,
        p010,
        p011,
        p000,
        p101,
        p110,
        p111,
    ])

    return box

def main():
    boxes_on = []
    boxes_off = []
    with open("../test", "r") as infile:
        for line in infile.readlines():
            state = line.startswith("on")
            coords = line.split(" ")[1].strip()
            c = coords.split(",")
            x = list(map(int,c[0][2:].split("..")))
            y = list(map(int,c[1][2:].split("..")))
            z = list(map(int,c[2][2:].split("..")))

            box = tobox(x,y,z)
            print(box)

            fig = plt.figure()
            ax = fig.add_subplot(projection="3d")
            ax.plot(*box.exterior.xyz)
            #if state:

            print(f"{x} {y} {z}")
            break
    pass

if __name__ == "__main__":
    main()