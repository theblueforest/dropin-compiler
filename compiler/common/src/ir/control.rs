/*     _              _ _
 *  __| |_ _ ___ _ __( |_)_ _
 * / _` | '_/ _ \ '_ \/| | ' \
 * \__,_|_| \___/ .__/ |_|_||_| dropin-compiler
 *              |_|
 * Copyright © 2019-2024 Blue Forest
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use super::Expression;

#[derive(Debug)]
pub enum Control {
  If(If),
  AnonymousFunction(AnonymousFunction),
  FunctionCall(FunctionCall),
}

#[derive(Debug)]
pub struct If {
  pub condition: Box<Expression>,
  pub then: Box<Expression>,
  pub else_: Option<Box<Expression>>,
}

#[derive(Debug)]
pub struct AnonymousFunction {
  pub args: Vec<String>,
  pub body: Box<Expression>,
}

#[derive(Debug)]
pub struct FunctionCall {
  pub function: Box<Expression>,
  pub args: Vec<Expression>,
}
