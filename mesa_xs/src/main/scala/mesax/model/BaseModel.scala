package mesax.model

import java.util.concurrent.atomic.AtomicInteger
import java.util.concurrent.atomic.AtomicReference
import scala.collection.immutable.HashMap
import mesax.base.BaseTypes._


/**
  * The trait that houses a model
  */
trait ModelDef {

  /**
    * Generate a UUID that's unique to the model
    *
    * @return a UUID
    */
  def genUUID(): UUID

  /**
    * Get a list of all the categories for the model
    *
    * @return an iterator that contains all the category names and UUID of each
    */
  def categories: Iterable[(String, UUID)]
}

class Model extends ModelDef {
  private val counter: AtomicInteger = new AtomicInteger()

  private val _categories: AtomicReference[HashMap[String, UUID]] = {

    val ret: AtomicReference[HashMap[String, UUID]] =
      new AtomicReference()
    ret.set(HashMap("Cat1" -> genUUID(), "Cat2" -> genUUID()))
    ret
  }
  def Model() {}
  def genUUID(): UUID = {
    val ret = counter.incrementAndGet()
    ret
  }

  def categories: Iterable[(String, UUID)] = _categories.get()
}
