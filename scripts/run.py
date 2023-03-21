import neutron


num_threads = 8
num_steps = 100

colour_map = [
    "3f007a",
    "7b0079",
    "aa0072",
    "d10065",
    "f01055",
    "ff4a42",
    "ff762b",
    "ffa004",
    "ffc900",
    "fff000",
]

num_neutrons = 1000000
block_size = 100

bump_dist = 1e-9
min_weight = 1e-3

gun_pos = [0.0, 0.0, -1.0]
gun_target = [0.0, 0.0, 0.0]
gun_spread = 30.0  # degrees

scat_coeff = 1e-1
abs_coeff = 1e-1
mins = [-0.5e-1, -0.5e-1, -0.5e-1]
maxs = [0.5e-1, 0.5e-1, 0.5e-1]

num_voxels = [128, 128, 128]

neutron.simulate(
    num_threads,
    num_steps,
    colour_map,
    num_neutrons,
    block_size,
    bump_dist,
    min_weight,
    gun_pos,
    gun_target,
    gun_spread,
    scat_coeff,
    abs_coeff,
    mins,
    maxs,
    num_voxels,
)
