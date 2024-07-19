
# Inspiration
DigiDist2 is a simple digital distortion plugin with only one parameter called "threhold".
This Plugin is made with the nih-plug framework available [here](https://github.com/robbert-vdh/nih-plug).
DigiDist2 is based on a tutorial made by Joe Clay. The Tutorial is available [here](https://seventeencups.net/posts/writing-an-audio-plugin-in-rust/).

# How to Build
To build the vst3 and the clap format of the plugin clone the repo 
`git clone https://github.com/MalekDeKalem/digidist2` 
and then change the directory to the clone repo
`cd digidist2`
now that you are in the root directory of the project execute the following command
`cargo xtask bundle digidist2 --release`
this will create a folder called "target" and in that folder there is going to be a folder called "bundled", in which you will find both the clap and also the vst3 plugin.
You can now move these plugins into whatever the folder is that gets scanned for plugins by your Pluginhost. In my case that would be in ~/.vst3 for vst3 plugins for example.
