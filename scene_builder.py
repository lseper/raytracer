import json

test_scene = {
    "aspect_ratio": 1.5,
    "image_width": 1200,
    "image_height": 800,
    "samples_per_pixel": 500,
    "camera": {
        "origin": {
            "vec": [13.0, 2.0, 3.0]
        },
        "lower_left_corner": {
            "vec": [3.0112371, -1.1992972, 2.6938112]
        },
        "horizontal": {
            "vec": [1.5556582, 0, -5.055889]
        },
        "vertical": {
            "vec": [-0.49034908, 3.4890225, -0.15087664]
        },
        "u": {
            "vec": [0.29408586, 0, -0.955779]
        },
        "v": {
            "vec": [-0.13904539, 0.9893614, -0.042783197]
        },
        "w": {
            "vec": [0.9456108, 0.14547859, 0.29095718]
        },
        "radius": 0.5
    },
    "world": [
        {
            "type": "Sphere",
                "center": {
                "vec": [0, -1000, 0] 
            },
            "r": 1000,
            "material": {
                "type": "Lambertian",
                "albedo": [0.5, 0.5, 0.5]
            } 
        },
        {
            "type": "Sphere",
                "center": {
                "vec": [0.49079672, 0.2, 0.49358273] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.047505677, 0.55955374, 0.68433684]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [0.3712137, 0.2, 1.5109301] 
            },
            "r": 0.2,
            "material": {
                "type": "Metal",
                "albedo": [0.66978514, 0.9735459, 0.52093863],
                "fuzziness": 0.045185298
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [0.108206265, 0.2, 2.8297648] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.9996424, 0.76519847, 0.5032934]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [0.84208643, 0.2, 3.2912557] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.25619048, 0.43671012, 0.6994946]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [0.47294956, 0.2, 4.3788357] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.5939847, 0.20641029, 0.34596628]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [0.74750704, 0.2, 5.7680845] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.68366766, 0.39869446, 0.036904454]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [0.34401453, 0.2, 6.7651477] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.33635843, 0.06357592, 0.53085965]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [0.024920404, 0.2, 7.366643] 
            },
            "r": 0.2,
            "material": {
                "type": "Metal",
                "albedo": [0.69441444, 0.779224, 0.86202097],
                "fuzziness": 0.47302094
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [0.034163617, 0.2, 8.46533] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.41729283, 0.31663805, 0.6072081]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [0.5606818, 0.2, 9.864729] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.03472948, 0.08596498, 0.25737357]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [0.098686785, 0.2, 10.256608] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.34667367, 0.56420857, 0.5180771]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [1.8656251, 0.2, 0.22322689] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.78944826, 0.6145177, 0.17013824]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [1.5600017, 0.2, 1.0346655] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.65407175, 0.42628956, 0.10218161]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [1.4313464, 0.2, 2.729107] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.56397134, 0.89486504, 0.70763355]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [1.8730656, 0.2, 3.1343398] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.6218393, 0.8012112, 0.99790746]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [1.6663268, 0.2, 4.3138566] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.70703155, 0.7000185, 0.5770453]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [1.4849968, 0.2, 5.222863] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.63934505, 0.95000976, 0.3373865]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [1.3526646, 0.2, 6.717757] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.5862227, 0.12298137, 0.38255686]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [1.1305134, 0.2, 7.1925006] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.96159405, 0.44479638, 0.93809354]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [1.7629756, 0.2, 8.460094] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.7032317, 0.9878729, 0.7280727]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [1.8167659, 0.2, 9.129732] 
            },
            "r": 0.2,
            "material": {
                "type": "Metal",
                "albedo": [0.5934612, 0.8056662, 0.83797216],
                "fuzziness": 0.36535144
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [1.1981006, 0.2, 10.79341] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.4018764, 0.31985295, 0.25996143]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [2.7235267, 0.2, 0.7358629] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.51800907, 0.6862935, 0.95047367]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [2.718334, 0.2, 1.0598241] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.10054898, 0.15302962, 0.23279011]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [2.028613, 0.2, 2.8354552] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.71158624, 0.5440049, 0.60105264]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [2.3194635, 0.2, 3.506108] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.36475623, 0.5195972, 0.09895575]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [2.47329, 0.2, 4.8120556] 
            },
            "r": 0.2,
            "material": {
                "type": "Metal",
                "albedo": [0.5908809, 0.596539, 0.77295154],
                "fuzziness": 0.32071912
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [2.107908, 0.2, 5.7678237] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.7862214, 0.6711142, 0.57726854]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [2.4487698, 0.2, 6.6771684] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.92723304, 0.21069127, 0.45344973]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [2.1961854, 0.2, 7.4867883] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.5201952, 0.7411375, 0.033293486]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [2.6125243, 0.2, 8.145942] 
            },
            "r": 0.2,
            "material": {
                "type": "Metal",
                "albedo": [0.7420198, 0.63058054, 0.835776],
                "fuzziness": 0.43007025
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [2.515273, 0.2, 9.77231] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.61392504, 0.50368875, 0.3606724]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [2.045056, 0.2, 10.353931] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.5836474, 0.5157691, 0.41734332]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [3.0115511, 0.2, 0.083312094] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.47783977, 0.7383458, 0.40960526]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [3.0419374, 0.2, 1.0950146] 
            },
            "r": 0.2,
            "material": {
                "type": "Metal",
                "albedo": [0.9079846, 0.652192, 0.645253],
                "fuzziness": 0.056579202
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [3.7037106, 0.2, 2.5426028] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.051333368, 0.86484575, 0.83757013]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [3.292137, 0.2, 3.1884763] 
            },
            "r": 0.2,
            "material": {
                "type": "Metal",
                "albedo": [0.80103576, 0.8254608, 0.5301901],
                "fuzziness": 0.36887667
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [3.0570993, 0.2, 4.3467193] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.0159899, 0.20771742, 0.8517153]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [3.807136, 0.2, 5.0240026] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.41797477, 0.43063003, 0.6171841]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [3.4307067, 0.2, 6.6747] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.67742187, 0.8031761, 0.0541991]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [3.172352, 0.2, 7.666244] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.8719913, 0.5116409, 0.71632457]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [3.747168, 0.2, 8.04441] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.70342845, 0.031330407, 0.35667384]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [3.4115562, 0.2, 9.195161] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.7705898, 0.22667354, 0.039396703]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [3.8177195, 0.2, 10.550076] 
            },
            "r": 0.2,
            "material": {
                "type": "Metal",
                "albedo": [0.69015414, 0.8466786, 0.9274051],
                "fuzziness": 0.10625395
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [4.5303497, 0.2, 1.236396] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.49844503, 0.48269695, 0.061460376]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [4.5623093, 0.2, 2.0277817] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.39228612, 0.944049, 0.07685876]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [4.6056542, 0.2, 3.608845] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.3419906, 0.55500513, 0.5883586]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [4.048792, 0.2, 4.5662003] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.6981269, 0.81852883, 0.65073526]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [4.3208227, 0.2, 5.291736] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.5776465, 0.007175505, 0.54561]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [4.5489945, 0.2, 6.653157] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.93369496, 0.73107874, 0.8154734]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [4.0614743, 0.2, 7.0001764] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.56868684, 0.74841034, 0.09174824]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [4.7727304, 0.2, 8.267131] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.7858359, 0.91634244, 0.668718]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [4.096772, 0.2, 9.82824] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.5754256, 0.94664335, 0.28230202]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [4.2486334, 0.2, 10.647376] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.64229566, 0.76977086, 0.15625232]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [5.600214, 0.2, 0.8831013] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.6342694, 0.7660125, 0.55416405]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [5.3702536, 0.2, 1.5835052] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.6105958, 0.8312899, 0.7487761]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [5.542945, 0.2, 2.1305244] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.35882145, 0.4458301, 0.27975172]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [5.82788, 0.2, 3.1484747] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.7362122, 0.26425314, 0.47061652]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [5.2513666, 0.2, 4.8202434] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.8149804, 0.42255652, 0.87330043]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [5.177032, 0.2, 5.8055162] 
            },
            "r": 0.2,
            "material": {
                "type": "Metal",
                "albedo": [0.8120562, 0.72300416, 0.5025007],
                "fuzziness": 0.09410274
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [5.607421, 0.2, 6.1427193] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.06280863, 0.021214604, 0.9202498]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [5.328415, 0.2, 7.054737] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.2762413, 0.6630132, 0.32765996]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [5.1720986, 0.2, 8.037448] 
            },
            "r": 0.2,
            "material": {
                "type": "Dielectric",
                "ir": 1.5
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [5.1089907, 0.2, 9.215155] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.20578593, 0.54079384, 0.95965075]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [5.113697, 0.2, 10.507784] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.64383733, 0.454368, 0.39163476]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [6.699096, 0.2, 0.105782405] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.17874509, 0.9834069, 0.89585966]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [6.2473245, 0.2, 1.0546415] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.25578314, 0.29703403, 0.22719449]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [6.8326125, 0.2, 2.432149] 
            },
            "r": 0.2,
            "material": {
                "type": "Metal",
                "albedo": [0.53192556, 0.790161, 0.73397064],
                "fuzziness": 0.13654956
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [6.1858764, 0.2, 3.2304149] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.8528665, 0.89893824, 0.10919678]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [6.8726983, 0.2, 4.1684294] 
            },
            "r": 0.2,
            "material": {
                "type": "Metal",
                "albedo": [0.6376121, 0.63406813, 0.7575617],
                "fuzziness": 0.4199989
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [6.50205, 0.2, 5.6294513] 
            },
            "r": 0.2,
            "material": {
                "type": "Dielectric",
                "ir": 1.5
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [6.4126625, 0.2, 6.4452944] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.7602111, 0.054008365, 0.7272247]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [6.3619127, 0.2, 7.2361884] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.044465303, 0.48431838, 0.9609163]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [6.4667063, 0.2, 8.543248] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.2949711, 0.1517852, 0.9708767]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [6.366299, 0.2, 9.192664] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.53190166, 0.7902184, 0.57773846]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [6.4252725, 0.2, 10.873792] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.9217457, 0.618732, 0.7855653]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [7.8745193, 0.2, 0.21869546] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.31472594, 0.102154315, 0.88402313]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [7.525407, 0.2, 1.5809741] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.13585156, 0.9975152, 0.88159484]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [7.1805573, 0.2, 2.5567966] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.92196935, 0.30588365, 0.6280412]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [7.1341114, 0.2, 3.0853076] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.58516455, 0.8501013, 0.45729846]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [7.669244, 0.2, 4.0417905] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.4484322, 0.6447192, 0.3652842]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [7.432204, 0.2, 5.062805] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.95312816, 0.41076487, 0.5496393]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [7.294726, 0.2, 6.288674] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.22631079, 0.107696176, 0.8492915]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [7.5031295, 0.2, 7.522203] 
            },
            "r": 0.2,
            "material": {
                "type": "Metal",
                "albedo": [0.5565182, 0.8760767, 0.9501666],
                "fuzziness": 0.26544538
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [7.0217505, 0.2, 8.127021] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.020109534, 0.39972568, 0.9421887]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [7.021243, 0.2, 9.361908] 
            },
            "r": 0.2,
            "material": {
                "type": "Metal",
                "albedo": [0.57166076, 0.6511668, 0.5277343],
                "fuzziness": 0.45228508
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [7.6189685, 0.2, 10.144906] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.49577296, 0.381077, 0.32266384]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [8.407282, 0.2, 0.7402505] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.21876955, 0.7695192, 0.23628592]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [8.748316, 0.2, 1.0721618] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.32723475, 0.18407601, 0.57968426]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [8.891982, 0.2, 2.295444] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.9218478, 0.35584342, 0.7155135]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [8.577227, 0.2, 3.504587] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.39098644, 0.16877067, 0.63619506]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [8.692078, 0.2, 4.5706363] 
            },
            "r": 0.2,
            "material": {
                "type": "Metal",
                "albedo": [0.8589014, 0.66878164, 0.6928855],
                "fuzziness": 0.3187576
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [8.298361, 0.2, 5.353164] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.6051341, 0.4611196, 0.274675]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [8.296304, 0.2, 6.458678] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.31773973, 0.24532229, 0.5776206]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [8.330961, 0.2, 7.6064663] 
            },
            "r": 0.2,
            "material": {
                "type": "Dielectric",
                "ir": 1.5
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [8.337076, 0.2, 8.789492] 
            },
            "r": 0.2,
            "material": {
                "type": "Dielectric",
                "ir": 1.5
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [8.121846, 0.2, 9.8622465] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.646815, 0.93340683, 0.9473971]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [8.647873, 0.2, 10.287624] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.32914102, 0.4684143, 0.41544956]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [9.358564, 0.2, 0.8198909] 
            },
            "r": 0.2,
            "material": {
                "type": "Metal",
                "albedo": [0.83343554, 0.5477563, 0.7124897],
                "fuzziness": 0.36560306
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [9.802176, 0.2, 1.7048559] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.11638397, 0.17329079, 0.596292]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [9.3058405, 0.2, 2.6060417] 
            },
            "r": 0.2,
            "material": {
                "type": "Metal",
                "albedo": [0.55001116, 0.99124914, 0.77351034],
                "fuzziness": 0.17621133
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [9.078489, 0.2, 3.5071826] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.34213686, 0.17536694, 0.7039455]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [9.898175, 0.2, 4.4020967] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.3370431, 0.4374383, 0.71090895]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [9.444963, 0.2, 5.711871] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.28298146, 0.7761701, 0.429084]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [9.5856695, 0.2, 6.106639] 
            },
            "r": 0.2,
            "material": {
                "type": "Metal",
                "albedo": [0.75665116, 0.8995267, 0.70793384],
                "fuzziness": 0.44067907
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [9.403898, 0.2, 7.3925066] 
            },
            "r": 0.2,
            "material": {
                "type": "Dielectric",
                "ir": 1.5
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [9.433299, 0.2, 8.261666] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.6473274, 0.8281902, 0.3710479]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [9.379477, 0.2, 9.094209] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.77318656, 0.56204826, 0.30119497]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [9.782489, 0.2, 10.612152] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.6083761, 0.3388185, 0.77644426]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [10.132367, 0.2, 0.29330894] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.3337053, 0.1413055, 0.9297902]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [10.044175, 0.2, 1.3067853] 
            },
            "r": 0.2,
            "material": {
                "type": "Metal",
                "albedo": [0.6479025, 0.7337353, 0.65644276],
                "fuzziness": 0.23707792
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [10.669088, 0.2, 2.7405798] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.7649921, 0.74306375, 0.27315223]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [10.782883, 0.2, 3.7205567] 
            },
            "r": 0.2,
            "material": {
                "type": "Metal",
                "albedo": [0.70789725, 0.6533027, 0.67752403],
                "fuzziness": 0.3858855
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [10.849419, 0.2, 4.0658984] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.7595174, 0.40066606, 0.8565304]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [10.302093, 0.2, 5.2639365] 
            },
            "r": 0.2,
            "material": {
                "type": "Metal",
                "albedo": [0.6287625, 0.5265176, 0.8491523],
                "fuzziness": 0.12878245
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [10.499803, 0.2, 6.816474] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.7582189, 0.34389973, 0.84167975]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [10.014208, 0.2, 7.721979] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.30800378, 0.5650242, 0.63971305]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [10.071528, 0.2, 8.07536] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.21710807, 0.75470465, 0.5585981]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [10.406518, 0.2, 9.469932] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.09666568, 0.19437546, 0.65490323]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [10.089756, 0.2, 10.313575] 
            },
            "r": 0.2,
            "material": {
                "type": "Lambertian",
                "albedo": [0.8039722, 0.90072453, 0.2646749]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [0, 1, 0] 
            },
            "r": 1,
            "material": {
                "type": "Dielectric",
                "ir": 1.5
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [-4, 1, 0] 
            },
            "r": 1,
            "material": {
                "type": "Lambertian",
                "albedo": [0.4, 0.2, 0.1]
            } 
        },

        {
            "type": "Sphere",
                "center": {
                "vec": [4, 1, 0] 
            },
            "r": 1,
            "material": {
                "type": "Metal",
                "albedo": [0.7, 0.6, 0.5],
                "fuzziness": 0
            } 
        },
    ],
}

if __name__ == "__main__":
    # Serializing json
    json_object = json.dumps(test_scene, indent=4)
    
    # Writing to sample.json
    with open("./scenes/test_scene_py.json", "w") as outfile:
        outfile.write(json_object)