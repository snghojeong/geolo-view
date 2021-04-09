import geolo_view
import sys
from PyQt5.QtWidgets import *
from PyQt5.QtGui import QFont
from PyQt5.QtGui import QFontDatabase


class MyApp(QWidget):

    def __init__(self):
        super().__init__()
        self.initUI()
        self.pos_list = list()
        self.pos_list.append(0)

        self.fltr_md = ""

    def initUI(self):
        self.lvle = QLineEdit()
        self.lvle.returnPressed.connect(self.filter_lv)

        self.mdle = QLineEdit()
        self.mdle.returnPressed.connect(self.filter_md)

        self.tb = QTextBrowser()
        self.tb.setAcceptRichText(True)
        self.tb.setOpenExternalLinks(True)
        fontdb = QFontDatabase()
        self.tb.setFont(fontdb.systemFont(QFontDatabase.FixedFont))

        self.prev_btn = QPushButton('Prev')
        self.prev_btn.pressed.connect(self.prev_logs)
        self.prev_btn.setSizePolicy(QSizePolicy.Fixed, QSizePolicy.Fixed)

        self.next_btn = QPushButton('Next')
        self.next_btn.pressed.connect(self.next_logs)
        self.next_btn.setSizePolicy(QSizePolicy.Fixed, QSizePolicy.Fixed)

        fltrbox = QHBoxLayout()
        fltrbox.addWidget(self.lvle, 0)
        fltrbox.addWidget(self.mdle, 0)

        btnbox = QHBoxLayout()
        btnbox.addWidget(self.prev_btn, 0)
        btnbox.addWidget(self.next_btn, 1)

        vbox = QVBoxLayout()
        vbox.addLayout(fltrbox, 0)
        vbox.addWidget(self.tb, 1)
        vbox.addLayout(btnbox, 2)

        self.setLayout(vbox)

        ret = geolo_view.read_log('jup.log', 0, 50)
        self.tb.append(ret["log"])
        self.prev_pos = 0
        self.next_pos = ret["pos"]

        self.setWindowTitle('QTextBrowser')
        self.setGeometry(100, 300, 1200, 300)
        self.show()

        scrollBar = self.tb.verticalScrollBar()
        scrollBar.setValue(0)

    def filter_lv(self):
        self.fltr_lv = self.lvle.text()
        self.tb.clear()
        ret = geolo_view.read_log('jup.log', 0, 50, lv=self.fltr_lv)
        self.tb.append(ret["log"])
        self.prev_pos = 0
        self.next_pos = ret["pos"]
        scrollBar = self.tb.verticalScrollBar()
        scrollBar.setValue(0)

    def filter_md(self):
        self.fltr_md = self.mdle.text()
        self.tb.clear()
        ret = geolo_view.read_log('jup.log', 0, 50, md=self.fltr_md)
        self.tb.append(ret["log"])
        self.prev_pos = 0
        self.next_pos = ret["pos"]
        scrollBar = self.tb.verticalScrollBar()
        scrollBar.setValue(0)

    def prev_logs(self):
        self.tb.clear()
        ret = geolo_view.read_log('jup.log', self.prev_pos, 50, md=self.fltr_md)
        self.tb.append(ret["log"])
        self.pos_list.pop()
        self.prev_pos = self.pos_list[-2]
        self.next_pos = ret["pos"]
        scrollBar = self.tb.verticalScrollBar()
        scrollBar.setValue(scrollBar.maximum())

    def next_logs(self):
        self.tb.clear()
        ret = geolo_view.read_log('jup.log', self.next_pos, 50, md=self.fltr_md)
        self.tb.append(ret["log"])
        self.prev_pos = self.pos_list[-1]
        self.pos_list.append(self.next_pos)
        self.next_pos = ret["pos"]
        scrollBar = self.tb.verticalScrollBar()
        scrollBar.setValue(0)

if __name__ == '__main__':
    app = QApplication(sys.argv)
    ex = MyApp()
    sys.exit(app.exec_())
