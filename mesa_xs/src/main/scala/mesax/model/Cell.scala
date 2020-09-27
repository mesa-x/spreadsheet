package mesax.model

import mesax.base.Value
import mesax.base.CategoryLabel
import mesax.base.BaseTypes._

/***
 * The classes, interfaces, etc. related to Cells
 * that hold values
 */

 class Cell(uuid: UUID, modelDef: ModelDef) {
     private var value: Value = null
     private var categoryLabels: Array[CategoryLabel] = Array()
 }

 object Cell {
     def newCell(modelDef: ModelDef): Cell = new Cell(modelDef.genUUID(), modelDef)
 }