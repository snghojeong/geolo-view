import geolo_view
import sys
from PyQt5.QtWidgets import *
from PyQt5.QtGui import QFont
from PyQt5.QtGui import QFontDatabase
from PyQt5.QtGui import QIcon


class MainWidget(QWidget):

    def __init__(self, fname = ""):
        super().__init__()

        self.filter_date = ''
        self.filter_lv = QLabel()
        self.filter_lv.setText('')
        self.filter_qlabel = ''
        self.filter_md = QLabel()
        self.filter_md.setText('')
        self.filter_msg = QLabel()
        self.filter_msg.setText('')

        self.initUI()

        self.pos_list = list()
        self.pos_list.append(0)

        self.fname = fname
        if self.fname != "":
            self.prev_pos = 0
            self.next_pos = self.load_file(self.fname, 0)
            scrollBar = self.tb.verticalScrollBar()
            scrollBar.setValue(0)

    def initUI(self):
        vbox = QVBoxLayout()

        # Filter Area
        self.date_le = QLineEdit()
        self.date_le.returnPressed.connect(self.apply_filter)
        self.date_cbox = QComboBox()
        self.date_cbox.setLineEdit(self.date_le)
        self.date_cbox.setSizePolicy(QSizePolicy.Fixed, QSizePolicy.Fixed)
        self.date_le.setText('DATE')
        #self.combo_box.addItems(["a", "b"])

        self.lvle = QLineEdit()
        self.lv_cbox = QComboBox()
        self.lv_cbox.setLineEdit(self.lvle)
        self.lv_cbox.setSizePolicy(QSizePolicy.Fixed, QSizePolicy.Fixed)
        self.lvle.setText('LEVEL')
        self.lvle.returnPressed.connect(self.apply_filter)
        self.lvle.textChanged.connect(self.filter_lv.setText)

        self.qlle = QLineEdit()
        self.qlle.returnPressed.connect(self.apply_filter)
        self.ql_cbox = QComboBox()
        self.ql_cbox.setLineEdit(self.qlle)
        self.ql_cbox.setSizePolicy(QSizePolicy.Fixed, QSizePolicy.Fixed)
        self.qlle.setText('QLABEL')

        self.mdle = QLineEdit()
        self.md_cbox = QComboBox()
        self.md_cbox.setLineEdit(self.mdle)
        self.md_cbox.setSizePolicy(QSizePolicy.Fixed, QSizePolicy.Fixed)
        self.mdle.setText('MODULE')
        self.mdle.returnPressed.connect(self.apply_filter)
        self.mdle.textChanged.connect(self.filter_md.setText)

        self.msgle = QLineEdit()
        self.msg_cbox = QComboBox()
        self.msg_cbox.setLineEdit(self.msgle)
        self.msgle.setText('MESSAGE')
        self.msgle.returnPressed.connect(self.apply_filter)
        self.msgle.textChanged.connect(self.filter_msg.setText)

        fltrbox = QHBoxLayout()
        fltrbox.addWidget(self.date_cbox, 0)
        fltrbox.addWidget(self.lv_cbox, 1)
        fltrbox.addWidget(self.ql_cbox, 2)
        fltrbox.addWidget(self.md_cbox, 3)
        fltrbox.addWidget(self.msg_cbox, 4)

        vbox.addLayout(fltrbox, 0)

        # Text browser
        self.tb = QTextBrowser()
        self.tb.setAcceptRichText(True)
        self.tb.setOpenExternalLinks(True)
        fontdb = QFontDatabase()
        self.tb.setFont(fontdb.systemFont(QFontDatabase.FixedFont))

        vbox.addWidget(self.tb, 1)

        # Button Area
        self.prev_btn = QPushButton('Prev')
        self.prev_btn.pressed.connect(self.prev_logs)
        self.prev_btn.setSizePolicy(QSizePolicy.Fixed, QSizePolicy.Fixed)

        self.next_btn = QPushButton('Next')
        self.next_btn.pressed.connect(self.next_logs)
        self.next_btn.setSizePolicy(QSizePolicy.Fixed, QSizePolicy.Fixed)

        btnbox = QHBoxLayout()
        btnbox.addWidget(self.prev_btn, 0)
        btnbox.addWidget(self.next_btn, 1)

        vbox.addLayout(btnbox, 2)

        self.setLayout(vbox)

        self.show()

    def load_file(self, fname, pos):
        self.tb.clear()
        ret = geolo_view.read_log(fname, pos, 100, 
                date=self.filter_date,
                lv=self.filter_lv.text(),
                qlabel=self.filter_qlabel,
                md=self.filter_md.text(),
                msg=self.filter_msg.text())
        self.tb.append(ret["log"])
        return ret["pos"];

    def apply_filter(self):
        if self.fname != "":
            self.prev_pos = 0
            self.next_pos = self.load_file(self.fname, 0)
            scrollBar = self.tb.verticalScrollBar()
            scrollBar.setValue(0)

    def prev_logs(self):
        if self.fname != "":
            self.next_pos = self.load_file(self.fname, self.prev_pos)
            self.prev_pos = self.pos_list[-2]
            self.pos_list.pop()
            scrollBar = self.tb.verticalScrollBar()
            scrollBar.setValue(scrollBar.maximum())

    def next_logs(self):
        if self.fname != "":
            self.prev_pos = self.pos_list[-1]
            self.pos_list.append(self.next_pos)
            self.next_pos = self.load_file(self.fname, self.next_pos)
            scrollBar = self.tb.verticalScrollBar()
            scrollBar.setValue(0)

class MyApp(QMainWindow):
    def __init__(self):
        super().__init__()
        self.initUI()

    def initUI(self):
        main_wg = MainWidget(self.showFileDlg())
        self.setCentralWidget(main_wg)

        # File chooser
        openFile = QAction(QIcon('open.png'), 'Open', self)
        openFile.setShortcut('Ctrl+O')
        openFile.triggered.connect(self.showFileDlg)

        menubar = self.menuBar()
        menubar.setNativeMenuBar(False)
        fileMenu = menubar.addMenu('&File')
        fileMenu.addAction(openFile)

        self.setWindowTitle('earth log viewer')
        self.setGeometry(100, 300, 1200, 1000)
        self.show()

    def showFileDlg(self):
        fnames = QFileDialog.getOpenFileName(self, 'Open file', './')
        return fnames[0]

if __name__ == '__main__':
    app = QApplication(sys.argv)
    ex = MyApp()
    sys.exit(app.exec_())
