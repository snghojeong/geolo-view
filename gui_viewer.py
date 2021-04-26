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
        vbox = QVBoxLayout()

        # Filter Area
        self.seq_label = QLabel()
        self.seq_label.setText("SEQ")
        self.seq_label.setSizePolicy(QSizePolicy.Fixed, QSizePolicy.Fixed)
        self.seqle = QLineEdit()
        self.seqle.returnPressed.connect(self.apply_filter)
        self.seq_cbox = QComboBox()
        self.seq_cbox.setLineEdit(self.seqle)
        self.seq_cbox.setSizePolicy(QSizePolicy.Fixed, QSizePolicy.Fixed)
        #self.combo_box.addItems(["a", "b"])

        self.lv_label = QLabel()
        self.lv_label.setText("LEVEL")
        self.lv_label.setSizePolicy(QSizePolicy.Fixed, QSizePolicy.Fixed)
        self.lvle = QLineEdit()
        self.lvle.returnPressed.connect(self.apply_filter)
        self.lv_cbox = QComboBox()
        self.lv_cbox.setLineEdit(self.lvle)
        self.lv_cbox.setSizePolicy(QSizePolicy.Fixed, QSizePolicy.Fixed)

        self.ql_label = QLabel()
        self.ql_label.setText("QLABEL")
        self.ql_label.setSizePolicy(QSizePolicy.Fixed, QSizePolicy.Fixed)
        self.qlle = QLineEdit()
        self.qlle.returnPressed.connect(self.apply_filter)
        self.ql_cbox = QComboBox()
        self.ql_cbox.setLineEdit(self.qlle)
        self.ql_cbox.setSizePolicy(QSizePolicy.Fixed, QSizePolicy.Fixed)

        self.md_label = QLabel()
        self.md_label.setText("MD")
        self.md_label.setSizePolicy(QSizePolicy.Fixed, QSizePolicy.Fixed)
        self.mdle = QLineEdit()
        self.mdle.returnPressed.connect(self.apply_filter)
        self.md_cbox = QComboBox()
        self.md_cbox.setLineEdit(self.mdle)
        self.md_cbox.setSizePolicy(QSizePolicy.Fixed, QSizePolicy.Fixed)

        self.msg_label = QLabel()
        self.msg_label.setText("MESSAGE")
        self.msgle = QLineEdit()
        self.msgle.returnPressed.connect(self.apply_filter)
        self.msg_cbox = QComboBox()
        self.msg_cbox.setLineEdit(self.msgle)

        fltrbox = QGridLayout()
        fltrbox.addWidget(self.seq_label, 0, 0)
        fltrbox.addWidget(self.seq_cbox, 1, 0)
        fltrbox.addWidget(self.lv_label, 0, 1)
        fltrbox.addWidget(self.lv_cbox, 1, 1)
        fltrbox.addWidget(self.ql_label, 0, 2)
        fltrbox.addWidget(self.ql_cbox, 1, 2)
        fltrbox.addWidget(self.md_label, 0, 3)
        fltrbox.addWidget(self.md_cbox, 1, 3)
        fltrbox.addWidget(self.msg_label, 0, 4)
        fltrbox.addWidget(self.msg_cbox, 1, 4)

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

        ret = geolo_view.read_log('jup.log', 0, 100)
        self.tb.append(ret["log"])
        self.prev_pos = 0
        self.next_pos = ret["pos"]

        self.setWindowTitle('QTextBrowser')
        self.setGeometry(100, 300, 1200, 300)
        self.show()

        scrollBar = self.tb.verticalScrollBar()
        scrollBar.setValue(0)

    def apply_filter(self):
        self.tb.clear()
        ret = geolo_view.read_log('jup.log', 0, 100, 
                seq=self.seqle.text(),
                lv=self.lvle.text(),
                qlabel=self.qlle.text(),
                tid=self.tidle.text(),
                md=self.mdle.text(),
                msg=self.msgle.text())
        self.tb.append(ret["log"])
        self.prev_pos = 0
        self.next_pos = ret["pos"]
        scrollBar = self.tb.verticalScrollBar()
        scrollBar.setValue(0)

    def prev_logs(self):
        self.tb.clear()
        ret = geolo_view.read_log('jup.log', self.prev_pos, 100, md=self.fltr_md)
        self.tb.append(ret["log"])
        self.pos_list.pop()
        self.prev_pos = self.pos_list[-2]
        self.next_pos = ret["pos"]
        scrollBar = self.tb.verticalScrollBar()
        scrollBar.setValue(scrollBar.maximum())

    def next_logs(self):
        self.tb.clear()
        ret = geolo_view.read_log('jup.log', self.next_pos, 100, md=self.fltr_md)
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
