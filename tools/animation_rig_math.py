"""Authoring-only fixed-length two-link solve shared by static pose tools."""
import math
import bpy
from mathutils import Vector,Matrix

def solve_two_link(rig,a_name,b_name,target,pole,max_flex):
    def update():bpy.context.view_layer.update()
    def head(name):return rig.matrix_world@rig.pose.bones[name].head
    def direction(name,a,b):
        rest=rig.data.bones[name];q=(rest.tail_local-rest.head_local).rotation_difference(b-a)@rest.matrix_local.to_quaternion()
        rig.pose.bones[name].matrix=rig.matrix_world.inverted()@(Matrix.Translation(a)@q.to_matrix().to_4x4());update()
    a=head(a_name);l1=rig.data.bones[a_name].length;l2=rig.data.bones[b_name].length;delta=target-a
    minimum=math.sqrt(l1*l1+l2*l2+2*l1*l2*math.cos(max_flex))+.001
    distance=max(minimum,min(delta.length,l1+l2-.002));axis=delta.normalized();target=a+axis*distance
    along=(l1*l1-l2*l2+distance*distance)/(2*distance);perp=pole-a-axis*(pole-a).dot(axis)
    if perp.length<1e-5:perp=axis.cross(Vector((1,0,0)))
    perp.normalize();joint=a+axis*along+perp*math.sqrt(max(0,l1*l1-along*along));direction(a_name,a,joint);direction(b_name,joint,target)
    return target
