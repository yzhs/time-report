import QtQuick 2.2
import QtQuick.Window 2.1
import QtQuick.Controls 1.3

ApplicationWindow {
	id: window
	visible: true
	title: "Abrechnung"

//	toolBar: ToolBar {
//		TextField {
//			id: searchBox
//
//			placeholderText: "Search..."
//			inputMethodHints: Qt.ImhNoPredictiveText
//
//			width: 123
//			anchors.right: parent.right
//			anchors.top: parent.top
//		}
//	}

	ListModel {
		id: studentModel
		ListElement {
			name: "Frederik Müller"
			date: "1.2.2017"
			start: "13:30"
			end: "15:00"
			week: "A"
			remark: ""
		}

		Component.onCompleted: {
			for (var i=0; i< 49; ++i)
			studentModel.append({"name":"", "date": "", "start": "", "end": "", "week": "", "remark": ""})
		}
	}

	Component {
		id: editableDelegate
		Item {
			property bool selected: false;

			Text {
				width: parent.width
				anchors.margins: 4
				anchors.left: parent.left
				anchors.verticalCenter: parent.verticalCenter
				elide: styleData.elideMode
				text: styleData.value !== undefined ? styleData.value : ""
				color: styleData.textColor
				visible: !styleData.selected
			}
			Loader {
				id: loaderEditor
				anchors.fill: parent
				anchors.margins: 4
				Connections {
					target: loaderEditor.item
					onAccepted: {
						studentModel.setProperty(styleData.row, styleData.role, loaderEditor.item.text)
					}
					onEditingFinished: {
						studentModel.setProperty(styleData.row, styleData.role, loaderEditor.item.text)
					}
				}
				sourceComponent: styleData.selected ? editor : null
				Component {
					id: editor
					TextInput {
						id: textInput
						color: styleData.textColor
						text: styleData.value
						MouseArea {
							id: mouseArea
							anchors.fill: parent
							hoverEnabled: true
							onClicked: textInput.forceActiveFocus()
						}
					}
				}
			}
		}
	}

	TableView {
		id: tableView
		model: studentModel

		width: parent.width
		height: parent.height

		TableViewColumn {
			role: "name"
			title: "Name"
			width: 150
		}
		TableViewColumn {
			role: "date"
			title: "Datum"
			width: 80
		}
		TableViewColumn {
			role: "start"
			title: "von"
		}
		TableViewColumn {
			role: "end"
			title: "bis"
		}
		TableViewColumn {
			role: "week"
			title: "Woche"
		}
		TableViewColumn {
			role: "remark"
			title: "Bemerkung"
		}

		itemDelegate: editableDelegate;
	}
}
