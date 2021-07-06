Running this with `cargo run` starts at about 42 FPS, and rapidly decreases, as memory usage slowly increases.

When doing a production build and running, FPS seems stable at 60 on my PC, but memory usage increases like in the debug
build.

This seems to be related to `textures.get_mut(tex_handle)` (line 30) in `src/main.rs`. Even if the actual modification
of the texture is commented out, memory usage grows.

```
Jul 06 11:57:40.454  INFO bevy diagnostic: fps                             :   43.606987  (avg 42.921899)
Jul 06 11:57:40.454  INFO bevy diagnostic: frame_count                     :   80.000000  (avg 80.000000)
Jul 06 11:57:41.440  INFO bevy diagnostic: frame_time                      :    0.023912s (avg 0.024363s)
Jul 06 11:57:41.440  INFO bevy diagnostic: fps                             :   42.000474  (avg 41.583643)
Jul 06 11:57:41.440  INFO bevy diagnostic: frame_count                     :  121.000000  (avg 121.000000)
Jul 06 11:57:42.435  INFO bevy diagnostic: frame_time                      :    0.024787s (avg 0.024912s)
Jul 06 11:57:42.435  INFO bevy diagnostic: fps                             :   40.244761  (avg 40.053711)
Jul 06 11:57:42.435  INFO bevy diagnostic: frame_count                     :  161.000000  (avg 161.000000)
Jul 06 11:57:43.454  INFO bevy diagnostic: frame_time                      :    0.025432s (avg 0.025634s)
Jul 06 11:57:43.454  INFO bevy diagnostic: fps                             :   39.531528  (avg 39.292647)
Jul 06 11:57:43.454  INFO bevy diagnostic: frame_count                     :  201.000000  (avg 201.000000)
Jul 06 11:57:44.434  INFO bevy diagnostic: frame_time                      :    0.025694s (avg 0.026790s)
Jul 06 11:57:44.434  INFO bevy diagnostic: fps                             :   38.313538  (avg 37.866782)
Jul 06 11:57:44.434  INFO bevy diagnostic: frame_count                     :  238.000000  (avg 238.000000)
Jul 06 11:57:45.441  INFO bevy diagnostic: frame_time                      :    0.027073s (avg 0.027330s)
Jul 06 11:57:45.441  INFO bevy diagnostic: fps                             :   36.944914  (avg 36.752299)
Jul 06 11:57:45.441  INFO bevy diagnostic: frame_count                     :  275.000000  (avg 275.000000)
Jul 06 11:57:46.435  INFO bevy diagnostic: frame_time                      :    0.027403s (avg 0.027658s)
Jul 06 11:57:46.435  INFO bevy diagnostic: fps                             :   36.327791  (avg 36.292401)
Jul 06 11:57:46.435  INFO bevy diagnostic: frame_count                     :  311.000000  (avg 311.000000)
Jul 06 11:57:47.461  INFO bevy diagnostic: frame_time                      :    0.029018s (avg 0.028707s)
Jul 06 11:57:47.461  INFO bevy diagnostic: fps                             :   35.429860  (avg 35.162240)
Jul 06 11:57:47.461  INFO bevy diagnostic: frame_count                     :  347.000000  (avg 347.000000)

Process finished with exit code -1

```