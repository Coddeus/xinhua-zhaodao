presets = {
    'data':                 ["e => eguo"],
    'imgh':                 1080,
    'imgw':                 1920,
    'positionh':            "top", # CSS-like
    'positionw':            "expand", # CSS-like
    'paddingh':             100,
    'paddingw':             100,
    'arrowposition':        "left",
    'horizontallisting':    True,
    'font':                 "YRDZST-Semibold.ttf",
    'bgcolor':              (255, 255, 255),
    'fontcolor':            (0, 0, 0),
    'filetype':             "pdf"
}

class Image:
    def __init__(self, data=presets['data'], imgh=presets['imgh'], imgw=presets['imgw'], positionh=presets['positionh'], positionw=presets['positionw'], paddingh=presets['paddingh'], paddingw=presets['paddingw'], arrowposition=presets['arrowposition'], horizontallisting=presets['horizontallisting'], font=presets['font'], bgcolor=presets['bgcolor'], fontcolor=presets['fontcolor'], filetype=presets['filetype']):

        self.data                   =   data
        self.imgh                   =   imgh
        self.imgw                   =   imgw
        self.positionh              =   positionh
        self.positionw              =   positionw
        self.paddingh               =   paddingh
        self.paddingw               =   paddingw
        self.arrowposition          =   arrowposition
        self.horizontallisting      =   horizontallisting
        self.font                   =   font
        self.bgcolor                =   bgcolor
        self.fontcolor              =   fontcolor
        self.filetype               =   filetype

        self.draw()
        self.write()
        self.export()

        return 0

    def draw(self):
        pass
    
    def write(self):
        pass
    
    def export(self):
        pass
