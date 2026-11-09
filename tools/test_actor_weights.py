#!/usr/bin/env python3
"""Focused authoring regressions for overlapping source volumes and hands."""
import unittest
from prepare_runtime_props import human_weights, vertex_weights


class ActorWeights(unittest.TestCase):
    def test_shared_field_and_female_source_height(self):
        for x,y in [(.23,1.45),(.29,1.16),(.32,.96),(.115,.55),(.105,.94)]:
            p=(x*1.23,y*1.23,0.)
            expected=human_weights(p)
            for name in ['torso','upper_arm','forearm','hand','fingers','pelvis','thigh','knee','calf']:
                self.assertEqual(vertex_weights(name,p,False),expected)
            j,w=human_weights((p[0],p[1]*1.69/1.82,p[2]),True)
            actual=dict(zip(j,w));reference=dict(zip(*expected))
            for index in set(actual)|set(reference):self.assertAlmostEqual(actual.get(index,0),reference.get(index,0))

    def test_wrists_do_not_fade_into_pelvis(self):
        for side in [-1,1]:
            for x in [.28,.30,.32,.35]:
                j,w=human_weights((side*x*1.23,.96*1.23,0.))
                self.assertAlmostEqual(dict(zip(j,w))[4 if side<0 else 6],1.)

    def test_outer_thigh_does_not_pick_up_an_arm(self):
        for side in [-1,1]:
            for x in [.12,.17,.205]:
                j,w=human_weights((side*x*1.23,.85*1.23,0.))
                self.assertTrue(all(weight==0. for index,weight in zip(j,w)if index in [3,4,5,6]))

    def test_field_continuity(self):
        for y in [.49,.55,.63,.88,.94,1.045,1.08,1.16,1.24,1.43,1.48,1.62]:
            for x in [.12,.2,.23,.265,.29,.32]:
                a=dict(zip(*human_weights((x*1.23,y*1.23-1e-6,0.))))
                b=dict(zip(*human_weights((x*1.23,y*1.23+1e-6,0.))))
                # Top-four truncation can discard tiny tails, so audit the whole
                # stress grid instead of only exact partition boundaries.
                self.assertLess(sum(abs(a.get(j,0)-b.get(j,0))for j in set(a)|set(b)),1e-3)

    def test_pelvis_midline_does_not_jump_between_legs(self):
        for y in [.75,.85,.94,1.02]:
            a=dict(zip(*human_weights((-1e-7,y*1.23,0.))))
            b=dict(zip(*human_weights((1e-7,y*1.23,0.))))
            self.assertLess(sum(abs(a.get(j,0)-b.get(j,0))for j in set(a)|set(b)),1e-6)

    def test_equipment_is_not_blended_tissue(self):
        for name in ['briefcase','tennis_racket','checker_flag','surfboard']:
            j,w=vertex_weights(name,(.5,1.,0.),False)
            self.assertEqual((j,w),((6,0,0,0),(1.,0.,0.,0.)))


if __name__=='__main__':unittest.main()
