package mesax.model

import java.util.concurrent.atomic.AtomicInteger
import java.util.concurrent.atomic.AtomicReference
import scala.collection.immutable.HashMap

/**
  * Types, etc.
  */
object ModelBase {
  type UUID = Int
}

/**
  * The trait that houses a model
  */
trait ModelDef {

  /**
    * Generate a UUID that's unique to the model
    *
    * @return a UUID
    */
  def genUUID(): ModelBase.UUID

  /**
    * Get a list of all the categories for the model
    *
    * @return an iterator that contains all the category names and UUID of each
    */
  def categories: Iterable[(String, ModelBase.UUID)]
}

class Model extends ModelDef {
  private val counter: AtomicInteger = new AtomicInteger()

  private val _categories: AtomicReference[HashMap[String, ModelBase.UUID]] = {

    val ret: AtomicReference[HashMap[String, ModelBase.UUID]] =
      new AtomicReference()
    ret.set(HashMap("Cat1" -> genUUID(), "Cat2" -> genUUID()))
    ret
  }
  def Model() {}
  def genUUID(): ModelBase.UUID = {
    val ret = counter.incrementAndGet()
    ret
  }

  def categories: Iterable[(String, ModelBase.UUID)] = _categories.get()
}
