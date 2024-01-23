import ifcopenshell
import ifcopenshell.geom
import ifcopenshell.util.shape
import mpl_toolkits.mplot3d as a3
import matplotlib.colors as colors
import pylab as pl
import numpy as np

ifc_file = ifcopenshell.open('samples/duplex.ifc')

settings = ifcopenshell.geom.settings()

ax = a3.Axes3D(pl.figure())
for ifcwall in ifc_file.by_type('IFCWALL'):
    shape = ifcopenshell.geom.create_shape(settings, ifcwall)
    matrix = ifcopenshell.util.shape.get_shape_matrix(shape)
    rotation = matrix[0:3,0:3]
    location = matrix[:,3][0:3]
    grouped_verts = ifcopenshell.util.shape.get_vertices(shape.geometry)
    grouped_faces = ifcopenshell.util.shape.get_faces(shape.geometry)

    for face in grouped_faces:
        vert_1 = np.matmul(rotation, np.array(grouped_verts[face[0]])) + location
        vert_2 = np.matmul(rotation, np.array(grouped_verts[face[1]])) + location
        vert_3 = np.matmul(rotation, np.array(grouped_verts[face[2]])) + location
        vtx = np.array((vert_1,vert_2,vert_3))
        tri = a3.art3d.Poly3DCollection([vtx])    
        tri.set_color(colors.rgb2hex(np.ones(3)/2.0))
        tri.set_edgecolor('k')
        ax.add_collection3d(tri)

pl.show()